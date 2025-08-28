//! Binary operations

use miette::SourceSpan;

use crate::dpscript::{ast::node::Node, check::CheckConst};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BinaryOpNode {
    pub span: SourceSpan,
    pub operation: BinaryOperation,
    pub lhs: Vec<Node>,
    pub rhs: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BinaryOperation {
    /// lhs + rhs
    Add,

    /// lhs - rhs
    Subtract,

    /// lhs * rhs
    Multiply,

    /// lhs / rhs
    Divide,
    
    /// lhs % rhs
    Modulo,

    /// lhs & rhs
    BitAnd,

    /// lhs | rhs
    BitOr,

    /// lhs ^ rhs
    BitXor,

    /// lhs && rhs
    CondAnd,

    /// lhs || rhs
    CondOr,

    /// lhs == rhs
    CondEq,

    /// lhs != rhs
    CondNeq,

    /// lhs > rhs
    CondGt,

    /// lhs >= rhs
    CondGe,

    /// lhs < rhs
    CondLt,

    /// lhs <= rhs
    CondLe,
}

impl CheckConst for BinaryOpNode {
    fn is_const(&self) -> bool {
        self.lhs.is_const() && self.rhs.is_const()
    }
}
