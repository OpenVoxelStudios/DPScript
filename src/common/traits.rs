use miette::SourceSpan;

use crate::{dpscript::ast::ast::Scope, util::Spanned};

pub trait Validated {
    /// Validate this node.
    fn validate(
        &self,
        scope: &Scope,
        warnings: &mut Vec<Spanned<String>>,
        errors: &mut Vec<Spanned<String>>,
    ) -> Result<(), ()>;
}

pub trait HasSpan {
    /// Get the SourceSpan for this node.
    /// This will clone the span.
    fn span(&self) -> SourceSpan;

    /// Get the span from this node, consuming it.
    fn into_span(self) -> SourceSpan;
}
