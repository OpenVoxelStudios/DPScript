use std::fmt;

use crate::{data::SourceSpan, node::Node};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct RefNode<'a> {
    pub span: SourceSpan,

    pub lhs: Box<Node<'a>>,
    pub data: RefData<'a>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum RefData<'a> {
    Ident(&'a str),
    ArrayIndex(Box<Node<'a>>),
    None,
}

impl<'a> fmt::Display for RefNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ref<{}, {}>", self.lhs, self.data)
    }
}

impl<'a> fmt::Display for RefData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefData::Ident(id) => write!(f, "ident<{id}>"),
            RefData::ArrayIndex(node) => write!(f, "array_index<{node}>"),
            RefData::None => write!(f, "none"),
        }
    }
}
