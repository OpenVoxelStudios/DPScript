use core::fmt;
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct TypeRef<'a> {
    pub data: TypeRefData<'a>,
    pub span: SourceSpan,
    pub resolved: Option<Box<ResolvedTypeRef<'a>>>,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet)]
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

    /// This means that `self.resolved` MUST be true~
    Resolved,
}

impl<'a> fmt::Display for TypeRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            TypeRefData::Named { name } => write!(f, "{}", name.0),
            TypeRefData::SizedArray { inner, length } => write!(f, "[{inner}; {length:?}]"),
            TypeRefData::UnsizedArray { inner } => write!(f, "[{inner}]"),

            TypeRefData::Resolved => {
                write!(f, "<inferred: {}>", self.resolved.as_ref().unwrap().data)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct ResolvedTypeRef<'a> {
    pub module: ModulePath<'a>,
    pub source_file: PathBuf,
    pub span: SourceSpan,
    pub data: TypeData<'a>,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpanGroup)]
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

impl<'a> fmt::Display for TypeData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Enum(it) => write!(f, "{}", it.name.0),
            Self::Struct(it) => write!(f, "{}", it.name.0),
            Self::Typedef(it) => write!(f, "{}", it.name.0),
        }
    }
}
