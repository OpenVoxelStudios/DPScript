use std::path::PathBuf;

use crate::{
    nodes::meta::DefFlags,
    prelude::{
        SourceSpan,
        def::{DefTrait, enums::Enum, structs::Struct},
        meta::DefMeta,
        value::Value,
    },
    util::{ModulePath, Name},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Typedef<'a> {
    pub name: Name<'a>,
    pub flags: Vec<DefFlags>,
    pub span: SourceSpan,
    pub meta: DefMeta<'a>,
}

impl<'a> DefTrait<'a> for Typedef<'a> {
    fn with_meta(mut self, meta: DefMeta<'a>) -> Self {
        self.meta = meta;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct TypeRef<'a> {
    pub data: TypeRefData<'a>,
    pub span: SourceSpan,
    pub resolved: Option<Box<ResolvedTypeRef<'a>>>,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet)]
pub enum TypeRefData<'a> {
    Named {
        name: Name<'a>,
    },

    SizedArray {
        inner: Box<TypeRef<'a>>,
        length: Box<Value<'a>>,
    },

    UnsizedArray {
        inner: Box<TypeRef<'a>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct ResolvedTypeRef<'a> {
    pub module: ModulePath<'a>,
    pub source_file: PathBuf,
    pub span: SourceSpan,
    pub data: TypeData<'a>,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpanGroup)]
pub enum TypeData<'a> {
    Enum(Enum<'a>),
    Struct(Struct<'a>),
    Typedef(Typedef<'a>),
}

impl<'a> TypeRef<'a> {
    pub fn void(span: SourceSpan) -> Self {
        Self {
            data: TypeRefData::Named {
                name: ("void", span),
            },
            span,
            resolved: None,
        }
    }
}
