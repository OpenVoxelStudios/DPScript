use crate::prelude::*;

pub trait HasSpan {
    /// Get the SourceSpan for this node.
    /// This will clone the span.
    fn span(&self) -> SourceSpan;

    /// Get the span from this node, consuming it.
    fn into_span(self) -> SourceSpan;
}
