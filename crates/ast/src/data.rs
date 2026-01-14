use std::ops;

use miette::LabeledSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SourceSpan {
    #[serde(skip)]
    pub start: usize,

    #[serde(skip)]
    pub end: usize,
}

pub type Spanned<T> = (T, SourceSpan);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NamedSource<'a> {
    pub file: &'a str,
    pub code: &'a str,
}

impl<'a> Into<SourceSpan> for pest::Span<'a> {
    fn into(self) -> SourceSpan {
        SourceSpan {
            start: self.start(),
            end: self.end(),
        }
    }
}

impl ops::Add<usize> for SourceSpan {
    type Output = SourceSpan;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            start: self.start,
            end: self.end + rhs,
        }
    }
}

impl SourceSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub trait HasSpan {
    /// Get the SourceSpan for this node.
    /// This will clone the span.
    fn span(&self) -> SourceSpan;

    /// Get the span from this node, consuming it.
    fn into_span(self) -> SourceSpan;
}

pub trait AddSpan {
    fn add(&self, other: SourceSpan) -> SourceSpan;
}

impl AddSpan for SourceSpan {
    #[allow(unused)]
    fn add(&self, other: SourceSpan) -> SourceSpan {
        SourceSpan::new(self.start.min(other.start), self.end.max(other.end))
    }
}

pub trait ExpandSpan {
    fn expand(&self, num: usize) -> SourceSpan;
}

impl ExpandSpan for SourceSpan {
    #[allow(unused)]
    fn expand(&self, num: usize) -> SourceSpan {
        SourceSpan::new(self.start, self.end + num)
    }
}

pub trait SpanUtil {
    fn label(&self) -> LabeledSpan;
}

impl SpanUtil for SourceSpan {
    fn label(&self) -> LabeledSpan {
        LabeledSpan::new(Some("here".into()), self.start, self.end - self.start)
    }
}

impl<'a> SpanUtil for pest::Span<'a> {
    fn label(&self) -> LabeledSpan {
        LabeledSpan::new(Some("here".into()), self.start(), self.end() - self.start())
    }
}
