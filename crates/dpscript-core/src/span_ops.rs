use miette::LabeledSpan;

use crate::SourceSpan;

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
