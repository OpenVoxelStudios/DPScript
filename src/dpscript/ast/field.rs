use crate::dpscript::{ast::ast::Scope, data::NodeInfo, ty::TypeRef};
use miette::SourceSpan;
use std::fmt;

/// This isn't a field accessor or value, it's a definition that a type even has a field at all.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct FieldNode {
    pub span: SourceSpan,

    /// Whether the field is public.
    pub is_public: bool,

    /// The field's owner.
    pub owner: TypeRef,

    /// The name of the field.
    pub name: String,

    /// The type of the field.
    pub ty: TypeRef,
}

impl NodeInfo for FieldNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        true
    }
}

impl fmt::Display for FieldNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let public = if self.is_public { "[public] " } else { "" };

        write!(
            f,
            "{}field [{}] -> {}: {};",
            public, self.owner, self.name, self.ty
        )
    }
}
