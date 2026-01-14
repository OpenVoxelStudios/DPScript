use std::fmt;
use crate::data::{SourceSpan, Spanned};

/// This isn't a field accessor or value, it's a definition that a type even has a field at all.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct FieldNode<'a> {
    pub span: SourceSpan,

    /// Whether the field is public.
    pub is_public: bool,

    /// The field's owner.
    pub owner: Spanned<&'a str>,

    /// The name of the field.
    pub name: Spanned<&'a str>,

    /// The type of the field.
    pub ty: Spanned<&'a str>,
}

impl<'a> fmt::Display for FieldNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let public = if self.is_public { "[public] " } else { "" };

        write!(
            f,
            "{}field [{}] -> {}: {};",
            public, self.owner.0, self.name.0, self.ty.0
        )
    }
}
