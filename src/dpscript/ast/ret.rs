use miette::SourceSpan;

use crate::dpscript::{ast::node::Node, data::NodeInfo};

use super::ast::Scope;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ReturnNode {
    pub span: SourceSpan,
    pub value: Option<Box<Node>>,
}

impl NodeInfo for ReturnNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }
}
