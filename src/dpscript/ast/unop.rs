//! Unary operations

use miette::SourceSpan;

use crate::dpscript::{ast::node::Node, check::CheckConst};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UnaryOpNode {
    pub span: SourceSpan,
    pub operation: UnaryOperation,
    pub value: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum UnaryOperation {
    /// value
    None,

    /// !value
    Negate,

    /// ~value
    BitNot,
}

impl CheckConst for UnaryOpNode {
    fn is_const(&self) -> bool {
        self.value.is_const()
    }
}
