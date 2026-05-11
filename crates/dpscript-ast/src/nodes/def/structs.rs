use crate::{
    nodes::{meta::DefFlags, types::TypeRef},
    prelude::{SourceSpan, def::DefTrait, meta::DefMeta},
    util::Name,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Struct<'a> {
    /// The name of the struct.
    pub name: Name<'a>,

    /// All the structs this struct extends.
    pub extends: Vec<TypeRef<'a>>,

    /// The definition flags of the struct.
    pub flags: Vec<DefFlags>,

    /// The struct's fields.
    pub fields: Vec<StructField<'a>>,

    /// The span the struct is defined in.
    pub span: SourceSpan,

    /// The definition's metadata.
    pub meta: DefMeta<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct StructField<'a> {
    pub name: Name<'a>,
    pub ty: TypeRef<'a>,
    pub span: SourceSpan,
    pub meta: DefMeta<'a>,
}

impl<'a> DefTrait<'a> for Struct<'a> {
    fn with_meta(mut self, meta: DefMeta<'a>) -> Self {
        self.meta = meta;
        self
    }
}
