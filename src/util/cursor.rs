use super::{HasSpan, bits::HasBits};
use crate::{
    Result,
    error::{TokenizerError, UnnamedTokenizerError},
    util::{FromBits, Spanned},
};
use miette::{NamedSource, SourceOffset, SourceSpan};

#[derive(Debug, Clone)]
pub struct Cursor<T: HasBits + Clone, M = ()> {
    src: T,
    inner: Vec<T::Bit>,
    pos: usize,
    meta: M,
}

impl<T: HasBits + Clone, M: Default> Cursor<T, M> {
    pub fn new(data: T) -> Self {
        Self {
            src: data.clone(),
            inner: data.get_bits(),
            pos: 0,
            meta: M::default(),
        }
    }
}

impl<T: HasBits + Clone> Cursor<T, NamedSource<String>> {
    pub fn new_from_src(file: impl AsRef<str>, code: impl AsRef<str>, data: T) -> Self {
        Self {
            src: data.clone(),
            inner: data.get_bits(),
            pos: 0,
            meta: NamedSource::new(file, code.as_ref().into()).with_language("dpscript"),
        }
    }
}

impl<T: HasBits + Clone> Cursor<T, String> {
    pub fn new_from_src(code: impl AsRef<str>, data: T) -> Self {
        Self {
            src: data.clone(),
            inner: data.get_bits(),
            pos: 0,
            meta: code.as_ref().into(),
        }
    }
}

impl<T: HasBits + Clone, M> Cursor<T, M> {
    pub fn is_empty(&self) -> bool {
        self.inner.len() <= self.pos
    }

    pub fn has_next(&self) -> bool {
        !self.is_empty()
    }

    pub fn next(&mut self) -> Option<T::Bit> {
        self.pos += 1;
        self.inner.get(self.pos - 1).cloned()
    }

    pub fn peek(&self) -> Option<T::Bit> {
        self.inner.get(self.pos).cloned()
    }

    pub fn peek_ahead(&self, num: usize) -> Option<T::Bit> {
        self.inner.get(self.pos + num).cloned()
    }

    pub fn skip(&mut self, num: usize) {
        self.pos += num;
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

impl<T: HasBits + Clone + FromBits, M> Cursor<T, M>
where
    T::Bit: PartialEq,
{
    pub fn next_group(&mut self, end: impl Fn(&T::Bit) -> bool) -> Option<T> {
        if !self.has_next() {
            return None;
        }

        let mut bits = Vec::new();

        while let Some(bit) = self.next() {
            if end(&bit) {
                self.pos -= 1;
                break;
            }

            bits.push(bit);
        }

        Some(T::from_bits(bits))
    }

    pub fn peek_group(&self, end: impl Fn(&T::Bit) -> bool) -> Option<T> {
        if !self.has_next() {
            return None;
        }

        let mut bits = Vec::new();
        let mut pos = 0;

        while let Some(bit) = self.peek_ahead(pos) {
            if end(&bit) {
                break;
            }

            bits.push(bit);
            pos += 1;
        }

        Some(T::from_bits(bits))
    }
}

impl<T: HasBits + Clone, M> Cursor<T, M>
where
    T::Bit: PartialEq,
{
    pub fn peek_until(&self, end: impl Fn(&T::Bit) -> bool) -> Option<T::Bit> {
        let mut it = None;
        let mut n = 0;

        while let Some(cur) = self.peek_ahead(n) {
            if end(&cur) {
                it = Some(cur);
                break;
            }

            n += 1;
        }

        it
    }

    pub fn peek_until_if(
        &self,
        end: impl Fn(&T::Bit) -> bool,
        check: impl Fn(&T::Bit) -> bool,
    ) -> Option<T::Bit> {
        let mut it = None;
        let mut n = 0;

        while let Some(cur) = self.peek_ahead(n) {
            if end(&cur) {
                it = Some(cur);
                break;
            }

            if !check(&cur) {
                break;
            }

            n += 1;
        }

        it
    }
}

impl<T: HasBits + Clone + FromIterator<T::Bit>, M> Cursor<T, M> {
    pub fn peek_many(&self, start: usize, num: usize) -> Option<T> {
        let mut parts = Vec::new();

        for i in 0..num {
            if let Some(bit) = self.peek_ahead(start + i) {
                parts.push(bit);
            } else {
                return None;
            }
        }

        Some(parts.iter().cloned().collect())
    }
}

impl<T: HasBits + Clone> Cursor<T, String> {
    pub fn source(&self) -> String {
        self.meta.clone()
    }
}

impl<B: Clone + HasSpan, T: HasBits<Bit = B> + Clone> Cursor<T, String> {
    pub fn next_or_die(&mut self) -> Result<T::Bit> {
        self.pos += 1;

        match self.inner.get(self.pos - 1).cloned() {
            Some(v) => Ok(v),
            None => Err(UnnamedTokenizerError {
                src: self.source(),
                at: self.inner.get(self.pos - 2).clone().unwrap().get_span(),
                err: "Unexpected end of file!".into(),
            }
            .into()),
        }
    }
}

impl<T: HasBits + Clone> Cursor<T, NamedSource<String>> {
    pub fn source(&self) -> NamedSource<String> {
        self.meta.clone()
    }

    pub fn next_or_die(&mut self, span: SourceSpan) -> Result<T::Bit> {
        self.pos += 1;

        match self.inner.get(self.pos - 1).cloned() {
            Some(v) => Ok(v),
            None => Err(TokenizerError {
                src: self.source(),
                at: span,
                err: "Unexpected end of file!".into(),
            }
            .into()),
        }
    }
}

impl Cursor<String, String> {
    pub fn new_from_code(data: impl AsRef<str>) -> Self {
        let s = data.as_ref().to_string();

        Self {
            src: s.clone(),
            inner: s.chars().collect(),
            pos: 0,
            meta: s,
        }
    }

    fn find_line(&self) -> usize {
        let mut lines = 0;

        for item in &self.inner[0..self.pos] {
            if *item == '\n' {
                lines += 1;
            }
        }

        lines
    }

    fn find_char(&self) -> usize {
        let line = self.find_line();
        let mut lines = 0;
        let mut chars = 0;

        for item in &self.inner[0..self.pos] {
            if *item == '\n' {
                lines += 1;
            } else {
                if line == lines {
                    chars += 1;
                }
            }
        }

        chars
    }

    pub fn span(&self, length: usize) -> SourceSpan {
        SourceSpan::new(
            SourceOffset::from_location(&self.src, self.find_line() + 1, self.find_char()),
            length,
        )
    }
}

impl Cursor<String, NamedSource<String>> {
    pub fn new_from_code(file: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        let s = data.as_ref().to_string();

        Self {
            inner: s.chars().collect(),
            meta: NamedSource::new(file, s.clone()).with_language("dpscript"),
            src: s,
            pos: 0,
        }
    }

    pub fn find_line(&self, pos: usize) -> usize {
        let mut lines = 0;

        for item in &self.inner[0..pos] {
            if *item == '\n' {
                lines += 1;
            }
        }

        lines
    }

    pub fn find_char(&self, pos: usize) -> usize {
        let line = self.find_line(pos);
        let mut lines = 0;
        let mut chars = 0;

        for item in &self.inner[0..pos] {
            if *item == '\n' {
                lines += 1;
            } else {
                if line == lines {
                    chars += 1;
                }
            }
        }

        chars
    }

    pub fn span(&self, length: usize) -> SourceSpan {
        SourceSpan::new(SourceOffset::from(self.pos), length)
    }

    pub fn span_prev(&self, length: usize, back: usize) -> SourceSpan {
        SourceSpan::new(SourceOffset::from(self.pos - back), length)
    }

    pub fn next_group_spanned(&mut self, end: impl Fn(&char) -> bool) -> Option<Spanned<String>> {
        if !self.has_next() {
            return None;
        }

        let mut bits = Vec::new();

        while let Some(bit) = self.next() {
            if end(&bit) {
                self.pos -= 1;
                break;
            }

            bits.push(bit);
        }

        let span = self.span_prev(bits.len(), bits.len());

        Some((String::from_bits(bits), span))
    }

    pub fn peek_group_spanned(&self, end: impl Fn(&char) -> bool) -> Option<Spanned<String>> {
        if !self.has_next() {
            return None;
        }

        let mut bits = Vec::new();
        let mut pos = 0;

        while let Some(bit) = self.peek_ahead(pos) {
            if end(&bit) {
                break;
            }

            bits.push(bit);
            pos += 1;
        }

        let span = self.span(bits.len());

        Some((String::from_bits(bits), span))
    }
}
