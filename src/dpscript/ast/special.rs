use std::fmt;

use dpscript_macros::HasSpan;
use miette::SourceSpan;

use crate::dpscript::{
    ast::{ast::Scope, nbt::NbtValue, node::Node},
    data::NodeInfo,
    ty::{BuiltInType, TypeRef, schema::TEXT_COMPONENT},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct SpecialNode {
    pub span: SourceSpan,
    pub data: SpecialData,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum SpecialData {
    Selector(String),
    Pos(Box<Node>, Box<Node>, Box<Node>),
    Component(NbtValue),
}

impl NodeInfo for SpecialNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        true
    }

    fn returns(&self, _scope: &Scope) -> Option<TypeRef> {
        Some(match self.data {
            SpecialData::Selector(_) => TypeRef::BuiltIn(BuiltInType::Selector),
            SpecialData::Pos(_, _, _) => TypeRef::BuiltIn(BuiltInType::Pos),
            SpecialData::Component(_) => TypeRef::TypedNBT(TEXT_COMPONENT.to_owned()),
        })
    }
}

impl fmt::Display for SpecialNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl fmt::Display for SpecialData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Selector(v) => write!(f, "selector<\"{v}\">"),
            Self::Pos(x, y, z) => write!(f, "pos<{x}, {y}, {z}>"),
            Self::Component(v) => write!(f, "{v}"),
        }
    }
}
