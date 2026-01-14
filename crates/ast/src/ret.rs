use crate::{data::SourceSpan, node::Node};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct ReturnNode<'a> {
    pub span: SourceSpan,
    pub value: Option<Box<Node<'a>>>,
}

impl<'a> fmt::Display for ReturnNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(it) => write!(f, "return {it};"),
            None => write!(f, "return;"),
        }
    }
}
