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
