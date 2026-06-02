use crate::SourceSpan;

pub struct StringCursor<'a> {
    inner: &'a str,
    // guaranteed to be ASCII
    inner_bytes: &'a [u8],
    pos: usize,
    peeker: usize,
    spans: Vec<usize>,

    state_stack: Vec<(usize, usize)>,
}

impl<'a> StringCursor<'a> {
    pub fn new(inner: &'a str) -> Self {
        Self {
            inner_bytes: inner.as_bytes(),
            inner,
            pos: 0,
            peeker: 0,
            spans: Vec::new(),
            state_stack: Vec::new(),
        }
    }

    pub fn back(&mut self) {
        self.pos -= 1;
        self.peeker = 0;
    }

    pub fn begin_span(&mut self) {
        self.spans.push(self.pos);
    }

    pub fn end_span(&mut self) -> SourceSpan {
        SourceSpan::new(self.spans.pop().unwrap(), self.pos)
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.pos + self.peeker < self.inner.len() {
            let res = self.inner_bytes[self.pos + self.peeker];
            self.peeker += 1;
            Some(res as char)
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.pos < self.inner.len() {
            self.peeker = 0;
            let res = Some(self.inner_bytes[self.pos] as char);
            self.pos += 1;
            res
        } else {
            None
        }
    }

    pub fn next_if<F: Fn(char) -> bool>(&mut self, f: F) -> Option<char> {
        if self.peek().is_some_and(|it| f(it)) {
            self.next()
        } else {
            self.peeker = self.peeker.saturating_sub(1);
            None
        }
    }

    pub fn next_if_eq(&mut self, eq: char) -> Option<char> {
        if self.peek().is_some_and(|it| it == eq) {
            Some(self.next().unwrap()) // essentially an assertion that it's not None because that wouldn't make sense anyway
        } else {
            self.peeker = self.peeker.saturating_sub(1);
            None
        }
    }

    pub fn take_while<F: Fn(char) -> bool>(&mut self, f: F) -> Vec<char> {
        let mut buf = Vec::new();

        while let Some(it) = self.next_if(&f) {
            buf.push(it);
        }

        buf
    }

    pub fn take(&mut self, count: usize) -> &'a str {
        let res = &self.inner[self.pos..self.pos + count];
        self.pos += count;
        self.peeker = 0;
        res
    }

    pub fn peek_many(&mut self, count: usize) -> Option<&'a str> {
        if self.pos + self.peeker + count >= self.inner.len() {
            return None;
        }

        let res = &self.inner[self.pos + self.peeker..self.pos + self.peeker + count];

        self.peeker += count;

        Some(res)
    }

    pub fn save(&mut self) {
        self.state_stack.push((self.pos, self.peeker));
    }

    pub fn restore(&mut self) {
        let (pos, peeker) = self.state_stack.pop().unwrap();

        self.pos = pos;
        self.peeker = peeker;
    }

    pub fn pop_state(&mut self) {
        self.state_stack.pop().unwrap();
    }

    pub fn clear_peeker(&mut self) {
        self.peeker = 0;
    }
}

pub struct CopyCursor<T> {
    inner: Vec<T>,
    pos: usize,
    peeker: usize,
    spans: Vec<usize>,

    state_stack: Vec<(usize, usize)>,
}

impl<T> CopyCursor<T> {
    pub fn new(iter: impl IntoIterator<Item = T>) -> Self {
        let inner: Vec<T> = iter.into_iter().collect();

        Self {
            inner,
            pos: 0,
            peeker: 0,
            spans: Vec::new(),
            state_stack: Vec::new(),
        }
    }
}

impl<T: Copy> CopyCursor<T> {
    pub fn begin_span(&mut self) {
        self.spans.push(self.pos);
    }

    pub fn end_span(&mut self) -> SourceSpan {
        SourceSpan::new(self.spans.pop().unwrap(), self.pos)
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.pos + self.peeker < self.inner.len() {
            let res = &self.inner[self.pos + self.peeker];
            self.peeker += 1;
            Some(&res)
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<T> {
        if self.pos < self.inner.len() {
            self.peeker = 0;
            let res = Some(self.inner[self.pos]);
            self.pos += 1;
            res.map(|it| it)
        } else {
            None
        }
    }

    pub fn next_if<F: Fn(&T) -> bool>(&mut self, f: F) -> Option<T> {
        if self.peek().is_some_and(|it| f(it)) {
            self.next()
        } else {
            None
        }
    }

    pub fn next_if_eq(&mut self, eq: &T) -> Option<T>
    where
        T: PartialEq,
    {
        if self.peek().is_some_and(|it| it == eq) {
            Some(self.next().unwrap()) // essentially an assertion that it's not None because that wouldn't make sense anyway
        } else {
            None
        }
    }

    pub fn take_while<F: Fn(&T) -> bool>(&mut self, f: F) -> Vec<T> {
        let mut buf = Vec::new();

        while let Some(it) = self.next_if(&f) {
            buf.push(it);
        }

        buf
    }

    pub fn take(&mut self, count: usize) -> Vec<T> {
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

    pub fn peek_many<const N: usize>(&mut self) -> Option<[T; N]>
    where
        T: Default,
    {
        if self.pos + self.peeker + N >= self.inner.len() {
            return None;
        }

        let mut arr = [T::default(); N];

        for i in 0..N {
            if let Some(it) = self.peek() {
                arr[i] = *it;
            }
        }

        Some(arr)
    }

    pub fn save(&mut self) {
        self.state_stack.push((self.pos, self.peeker));
    }

    pub fn restore(&mut self) {
        let (pos, peeker) = self.state_stack.pop().unwrap();

        self.pos = pos;
        self.peeker = peeker;
    }

    pub fn pop_state(&mut self) {
        self.state_stack.pop().unwrap();
    }
}
