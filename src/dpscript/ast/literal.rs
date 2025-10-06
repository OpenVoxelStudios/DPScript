use dpscript_macros::HasSpan;
use miette::SourceSpan;
use std::fmt;

use crate::dpscript::{
    ast::{ast::Scope, nbt::NbtValue, node::Node},
    data::NodeInfo,
    ty::{BuiltInType, TypeRef},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct LiteralNode {
    pub span: SourceSpan,
    pub data: LiteralData,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum LiteralData {
    String(String),
    Int(i64),
    Float(f32),
    Double(f64),
    Bool(bool),
    Array(Vec<Node>),
    Nbt(NbtValue),
}

impl NodeInfo for LiteralNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        true
    }

    fn returns(&self, scope: &Scope) -> Option<TypeRef> {
        Some(match self.data {
            LiteralData::String(_) => TypeRef::BuiltIn(BuiltInType::String),
            LiteralData::Int(_) => TypeRef::BuiltIn(BuiltInType::Int),
            LiteralData::Float(_) => TypeRef::BuiltIn(BuiltInType::Float),
            LiteralData::Double(_) => TypeRef::BuiltIn(BuiltInType::Double),
            LiteralData::Bool(_) => TypeRef::BuiltIn(BuiltInType::Boolean),
            LiteralData::Nbt(_) => TypeRef::BuiltIn(BuiltInType::NBT),

            LiteralData::Array(ref data) => {
                TypeRef::Array(data.iter().map(|it| it.returns(scope)).collect())
            }
        })
    }
}

impl fmt::Display for LiteralNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl fmt::Display for LiteralData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(v) => write!(f, "\"{v}\""),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}f"),
            Self::Double(v) => write!(f, "{v}d"),
            Self::Bool(v) => write!(f, "{v}"),
            Self::Nbt(v) => write!(f, "nbt<{v}>"),

            Self::Array(v) => write!(
                f,
                "[{}]",
                v.iter()
                    .map(|it| format!("{it}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
