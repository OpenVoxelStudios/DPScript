use miette::SourceOffset;
use std::ops::Add;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet, Default,
)]
pub struct SourceSpan {
    #[serde(skip)]
    pub start: usize,

    #[serde(skip)]
    pub end: usize,
}

impl SourceSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }

    pub fn position(&self, src: &str) -> (usize, usize) {
        let mut pos = 0;
        let mut line = 0;
        let mut post = 0;

        for ch in src.chars() {
            if pos > self.start {
                break;
            }

            pos += 1;
            post += 1;

            if ch == '\n' {
                post = 0;
                line += 1;
            }
        }

        (line, post)
    }
}

impl<'a> Into<SourceSpan> for pest::Span<'a> {
    fn into(self) -> SourceSpan {
        SourceSpan {
            start: self.start(),
            end: self.end(),
        }
    }
}

impl Into<miette::SourceSpan> for SourceSpan {
    fn into(self) -> miette::SourceSpan {
        miette::SourceSpan::new(SourceOffset::from(self.start), self.end - self.start)
    }
}

impl Add<usize> for SourceSpan {
    type Output = SourceSpan;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            start: self.start,
            end: self.end + rhs,
        }
    }
}

impl Add<SourceSpan> for SourceSpan {
    type Output = SourceSpan;

    fn add(self, rhs: SourceSpan) -> Self::Output {
        Self {
            start: self.start,
            end: self.end + (rhs.end - rhs.start),
        }
    }
}
