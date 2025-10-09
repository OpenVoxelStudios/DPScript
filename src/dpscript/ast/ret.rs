use std::fmt;

use miette::SourceSpan;

use crate::dpscript::{ast::node::Node, data::NodeInfo};

use super::ast::Scope;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct ReturnNode {
    pub span: SourceSpan,
    pub value: Option<Box<Node>>,
}

impl NodeInfo for ReturnNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }
}

impl fmt::Display for ReturnNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(it) => write!(f, "return {it};"),
            None => write!(f, "return;"),
        }
    }
}
