use crate::{
    prelude::{
        SourceSpan,
        def::DefTrait,
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

impl<'a> DefTrait<'a> for Constant<'a> {
    fn with_meta(mut self, meta: DefMeta<'a>) -> Self {
        self.meta = meta;
        self
    }
}
