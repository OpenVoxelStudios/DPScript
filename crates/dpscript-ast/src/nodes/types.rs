use core::fmt;

use crate::{
    nodes::meta::DefFlags,
    prelude::{
        SourceSpan,
        def::{DefTrait, enums::Enum, structs::Struct},
        meta::DefMeta,
        value::{
            Value,
            literal::{Literal, LiteralValue},
        },
    },
    util::Name,
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
    pub module: String,
    pub span: SourceSpan,
    pub data: TypeData<'a>,
    pub array: ArrayKind,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet, Default,
)]
pub enum ArrayKind {
    #[default]
    None,
    Sized,
    Unsized,
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

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Facet, Hash)]
pub enum TypeRefId {
    Named {
        name: (String, SourceSpan),
    },

    SizedArray {
        inner: Box<TypeRefId>,
        length: usize,
    },

    UnsizedArray {
        inner: Box<TypeRefId>,
    },
}

impl<'a> TypeRef<'a> {
    pub fn as_id(&self) -> TypeRefId {
        match &self.resolved {
            Some(resolved) => {
                let name = match &resolved.data {
                    TypeData::Enum(it) => {
                        (format!("{}::{}", resolved.module, it.name.0), it.name.1)
                    }

                    TypeData::Struct(it) => {
                        (format!("{}::{}", resolved.module, it.name.0), it.name.1)
                    }

                    TypeData::Typedef(it) => {
                        (format!("{}::{}", resolved.module, it.name.0), it.name.1)
                    }
                };

                match resolved.array {
                    ArrayKind::None => TypeRefId::Named { name },

                    _ => TypeRefId::UnsizedArray {
                        inner: Box::new(TypeRefId::Named { name }),
                    },
                }
            }

            None => match &self.data {
                TypeRefData::Named { name } => TypeRefId::Named {
                    name: (name.0.into(), name.1),
                },

                TypeRefData::SizedArray { inner, length } => match &**length {
                    Value::Literal(Literal {
                        value: LiteralValue::Int(len),
                        ..
                    }) => TypeRefId::SizedArray {
                        inner: Box::new(inner.as_id()),
                        length: *len as usize,
                    },

                    _ => TypeRefId::UnsizedArray {
                        inner: Box::new(inner.as_id()),
                    },
                },

                TypeRefData::UnsizedArray { inner } => TypeRefId::UnsizedArray {
                    inner: Box::new(inner.as_id()),
                },

                TypeRefData::Resolved => {
                    unreachable!("TypeRefData::Resolved is not valid if self.resolved is None!")
                }
            },
        }
    }
}
