use miette::SourceOffset;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
