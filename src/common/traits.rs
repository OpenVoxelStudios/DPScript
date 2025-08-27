use crate::util::Spanned;

pub trait Validated {
    /// Validate this node.
    fn validate(
        &self,
        module: &Module,
        warnings: &mut Vec<Spanned<String>>,
        errors: &mut Vec<Spanned<String>>,
    ) -> Result<(), ()>;
}
