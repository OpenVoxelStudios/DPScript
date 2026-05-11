use std::fmt;

use crate::{Result, err::Error};
use dpscript_core::{SourceSpan, Spanned};
use dpscript_parser::{BraceType, Literal, Token};

pub struct TokenCursor<'a> {
    inner: Vec<Spanned<Token<'a>>>,
    pos: usize,
    peeker: usize,
    spans: Vec<SourceSpan>,

    state_stack: Vec<(usize, usize, Vec<SourceSpan>)>,
}

impl<'a> fmt::Debug for TokenCursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenCursor")
            .field("pos", &self.pos)
            .field("peeker", &self.peeker)
            .finish()
    }
}

impl<'a> fmt::Display for TokenCursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Something better but this is just for debugging

        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(|it| format!("{}", it.0))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl<'a> TokenCursor<'a> {
    pub fn new(iter: impl IntoIterator<Item = Spanned<Token<'a>>>) -> Self {
        let inner: Vec<Spanned<Token<'a>>> = iter.into_iter().collect();

        Self {
            inner,
            pos: 0,
            peeker: 0,
            spans: Vec::new(),
            state_stack: Vec::new(),
        }
    }

    pub fn cur_span(&self) -> SourceSpan {
        if self.inner.is_empty() {
            return SourceSpan { start: 0, end: 0 };
        }

        if self.pos <= 0 {
            self.inner[self.pos].1
        } else if self.pos <= self.inner.len() {
            self.inner[self.pos - 1].1
        } else {
            panic!(
                "cursor out of bounds: position {}, length {}",
                self.pos,
                self.inner.len()
            )
        }
    }

    pub fn prev_span(&self) -> SourceSpan {
        if self.pos <= 1 {
            self.inner[0].1
        } else if self.pos <= self.inner.len() {
            self.inner[self.pos - 2].1
        } else {
            panic!(
                "cursor out of bounds: position {}, length {}",
                self.pos,
                self.inner.len()
            )
        }
    }

    pub fn begin_span(&mut self) {
        self.spans.push(self.cur_span());
    }

    pub fn begin_span_prev(&mut self) {
        self.spans.push(self.prev_span());
    }

    pub fn end_span(&mut self) -> SourceSpan {
        let start = self.spans.pop().unwrap();
        let end = self.cur_span();

        start + end
    }

    pub fn peek(&mut self) -> Option<&Token<'a>> {
        if self.pos + self.peeker < self.inner.len() {
            let res = &self.inner[self.pos + self.peeker];
            self.peeker += 1;
            Some(&res.0)
        } else {
            None
        }
    }

    pub fn peek_full(&mut self) -> Option<&Spanned<Token<'a>>> {
        if self.pos + self.peeker < self.inner.len() {
            let res = &self.inner[self.pos + self.peeker];
            self.peeker += 1;
            Some(&res)
        } else {
            None
        }
    }

    pub fn peek_in(&mut self, num: usize) -> Option<&Token<'a>> {
        if self.pos + self.peeker + num < self.inner.len() {
            let res = &self.inner[self.pos + self.peeker + num - 1];
            self.peeker += num;
            Some(&res.0)
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<Spanned<Token<'a>>> {
        if self.pos < self.inner.len() {
            self.peeker = 0;
            let res = Some(self.inner[self.pos]);
            self.pos += 1;
            res
        } else {
            None
        }
    }

    pub fn back(&mut self) {
        self.pos = self.pos.saturating_sub(1);
    }

    pub fn next_if<F: Fn(&Token<'a>) -> bool>(&mut self, f: F) -> Option<Spanned<Token<'a>>> {
        if self.peek().is_some_and(|it| f(it)) {
            self.next()
        } else {
            self.peeker = self.peeker.saturating_sub(1);
            None
        }
    }

    pub fn next_if_eq(&mut self, eq: &Token<'a>) -> Option<Spanned<Token<'a>>> {
        if self.peek().is_some_and(|it| it == eq) {
            Some(self.next().unwrap()) // essentially an assertion that it's not None because that wouldn't make sense anyway
        } else {
            self.peeker = self.peeker.saturating_sub(1);
            None
        }
    }

    pub fn has_next(&self) -> bool {
        self.pos < self.inner.len()
    }

    pub fn take_next(&mut self) -> Result<Spanned<Token<'a>>> {
        if self.pos < self.inner.len() {
            self.peeker = 0;
            let res = Ok(self.inner[self.pos]);
            self.pos += 1;
            res
        } else {
            Err(Error::Eof {
                span: self.cur_span().into(),
            })
        }
    }

    pub fn take_while<F: Fn(&Token<'a>) -> bool>(&mut self, f: F) -> Vec<Spanned<Token<'a>>> {
        let mut buf = Vec::new();

        while let Some(it) = self.next_if(&f) {
            buf.push(it);
        }

        buf
    }

    pub fn take(&mut self, count: usize) -> Vec<Spanned<Token<'a>>> {
        let mut buf = Vec::new();

        for _ in 0..count {
            if let Some(it) = self.next() {
                buf.push(it);
            } else {
                break;
            }
        }

        buf
    }

    pub fn save(&mut self) {
        self.state_stack
            .push((self.pos, self.peeker, self.spans.clone()));
    }

    pub fn restore(&mut self) {
        let (pos, peeker, spans) = self.state_stack.pop().unwrap();

        self.pos = pos;
        self.peeker = peeker;
        self.spans = spans;
    }

    pub fn pop_state(&mut self) {
        self.state_stack.pop().unwrap();
    }

    pub fn expect(&mut self, token: Token<'a>) -> Result<Spanned<Token<'a>>> {
        match self.peek_full() {
            Some(it) => {
                if it.0 == token {
                    Ok(self.next().unwrap())
                } else {
                    Err(Error::UnexpectedToken {
                        token: it.0.into(),
                        span: it.1.into(),
                    })
                }
            }

            None => Err(Error::Eof {
                span: self.cur_span().into(),
            }),
        }
    }

    pub fn expect_or_skip(&mut self, token: Token<'a>) -> Result<Spanned<Token<'a>>> {
        self.expect(token).map_err(|_| Error::Skip)
    }

    pub fn expect_group(&mut self, kind: BraceType) -> Result<TokenCursor<'a>> {
        match self.peek_full() {
            Some(it) => {
                if let (Token::BraceGroup(k, _), _) = it
                    && *k == kind
                {
                    let (Token::BraceGroup(_, arr), _) = self.next().unwrap() else {
                        unreachable!();
                    };

                    Ok(TokenCursor::new(arr.to_vec()))
                } else {
                    Err(Error::UnexpectedToken {
                        token: it.0.into(),
                        span: it.1.into(),
                    })
                }
            }

            None => Err(Error::Eof {
                span: self.cur_span().into(),
            }),
        }
    }

    pub fn expect_ident(&mut self) -> Result<Spanned<&'a str>> {
        match self.peek_full() {
            Some((Token::Literal(Literal::Identifier(_)), _)) => {
                let Some((Token::Literal(Literal::Identifier(id)), span)) = self.next() else {
                    unreachable!();
                };

                Ok((id, span))
            }

            Some(it) => Err(Error::UnexpectedToken {
                token: it.0.into(),
                span: it.1.into(),
            }),

            None => Err(Error::Eof {
                span: self.cur_span().into(),
            }),
        }
    }

    pub fn expect_str(&mut self) -> Result<Spanned<&'a str>> {
        match self.peek_full() {
            Some((Token::Literal(Literal::String(_)), _)) => {
                let Some((Token::Literal(Literal::String(str)), span)) = self.next() else {
                    unreachable!();
                };

                Ok((str, span))
            }

            Some(it) => Err(Error::UnexpectedToken {
                token: it.0.into(),
                span: it.1.into(),
            }),

            None => Err(Error::Eof {
                span: self.cur_span().into(),
            }),
        }
    }

    pub fn assert_empty(&mut self) -> Result<()> {
        if self.pos < self.inner.len() {
            Err(Error::UnexpectedToken {
                token: self.inner[self.pos].0.into(),
                span: self.inner[self.pos].1.into(),
            })
        } else {
            Ok(())
        }
    }

    pub fn next_if_ident(&mut self, ident: &'a str) -> Option<Spanned<Token<'a>>> {
        self.next_if_eq(&Token::Literal(Literal::Identifier(ident)))
    }

    pub fn clear_peek(&mut self) {
        self.peeker = 0;
    }
}
