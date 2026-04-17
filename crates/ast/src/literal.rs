use crate::{data::SourceSpan, nbt::NbtValue, node::Node};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct LiteralNode<'a> {
    pub span: SourceSpan,
    pub data: LiteralData<'a>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum LiteralData<'a> {
    String(&'a str),
    Int(i64),
    Float(f32),
    Double(f64),
    Bool(bool),
    Array(Vec<Node<'a>>),
    Nbt(NbtValue<'a>),
    Ident(&'a str),
}

impl<'a> LiteralData<'a> {
    pub fn as_string(self) -> Option<&'a str> {
        match self {
            LiteralData::String(it) => Some(it),
            _ => None,
        }
    }

    pub fn as_ident(self) -> Option<&'a str> {
        match self {
            LiteralData::Ident(it) => Some(it),
            _ => None,
        }
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, LiteralData::Ident(_))
    }
}

impl<'a> fmt::Display for LiteralNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<'a> fmt::Display for LiteralData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(v) => write!(f, "\"{v}\""),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}f"),
            Self::Double(v) => write!(f, "{v}d"),
            Self::Bool(v) => write!(f, "{v}"),
            Self::Nbt(v) => write!(f, "nbt<{v}>"),
            Self::Ident(v) => write!(f, "ident<{v}>"),

            Self::Array(v) => write!(
                f,
                "array<[{}]>",
                v.iter()
                    .map(|it| format!("{it}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
