use crate::{data::SourceSpan, nbt::NbtValue, node::Node};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct SpecialNode<'a> {
    pub span: SourceSpan,
    pub data: SpecialData<'a>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum SpecialData<'a> {
    Selector(&'a str),
    Pos(Box<Node<'a>>, Box<Node<'a>>, Box<Node<'a>>),
    Component(NbtValue<'a>),
}

impl<'a> fmt::Display for SpecialNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<'a> fmt::Display for SpecialData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Selector(v) => write!(f, "selector<\"{v}\">"),
            Self::Pos(x, y, z) => write!(f, "pos<{x}, {y}, {z}>"),
            Self::Component(v) => write!(f, "{v}"),
        }
    }
}
