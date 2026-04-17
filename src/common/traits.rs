use ast::{
    data::{SourceSpan, Spanned},
    scope::Scope,
};

pub trait Validated<'a> {
    /// Validate this node.
    fn validate(
        &self,
        scope: &Scope<'a>,
        warnings: &mut Vec<Spanned<&'a str>>,
        errors: &mut Vec<Spanned<&'a str>>,
    ) -> Result<(), ()>;
}

pub trait HasSpan {
    /// Get the SourceSpan for this node.
    /// This will clone the span.
    fn span(&self) -> SourceSpan;

    /// Get the span from this node, consuming it.
    fn into_span(self) -> SourceSpan;
}
