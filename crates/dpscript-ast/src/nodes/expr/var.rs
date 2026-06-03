use crate::{
    prelude::{SourceSpan, types::TypeRef, value::Value},
    util::Name,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct Variable<'a> {
    pub name: Name<'a>,
    pub ty: Option<TypeRef<'a>>,
    pub value: Option<Value<'a>>,
    pub span: SourceSpan,
}
