use crate::{
    prelude::{
        SourceSpan,
        meta::{DefFlags, DefMeta},
        types::TypeRef,
        value::Value,
    },
    util::Name,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Constant<'a> {
    pub name: Name<'a>,
    pub ty: TypeRef<'a>,
    pub value: Value<'a>,
    pub span: SourceSpan,
    pub meta: DefMeta<'a>,
    pub flags: Vec<DefFlags>,
}
