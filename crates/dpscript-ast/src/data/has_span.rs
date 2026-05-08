use crate::prelude::*;

pub trait HasSpan {
    /// Get the SourceSpan for this node.
    /// This will clone the span.
    fn span(&self) -> SourceSpan;

    /// Get the span from this node, consuming it.
    fn into_span(self) -> SourceSpan;
}

impl<T> HasSpan for (T, SourceSpan) {
    fn span(&self) -> SourceSpan {
        self.1
    }

    fn into_span(self) -> SourceSpan {
        self.1
    }
}
