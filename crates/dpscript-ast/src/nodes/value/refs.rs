use dpscript_core::SourceSpan;

use crate::{
    prelude::{types::TypeRef, value::Value},
    util::{Name, Remote},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct ValueRef<'a> {
    /// The value root.
    /// If this was `(a + b).c.d.e`, this would be `(a + b)`.
    pub root: Box<Value<'a>>,

    /// The field path.
    /// If this was `(a + b).c.d.e`, this would be [c, d, e].
    pub path: Vec<Name<'a>>,

    /// The span of the value ref.
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct VarRef<'a> {
    pub name: Name<'a>,
    pub resolved: Option<Remote<TypeRef<'a>>>,
    pub span: SourceSpan,
}
