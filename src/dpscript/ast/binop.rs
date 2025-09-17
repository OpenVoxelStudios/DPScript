//! Binary operations

use dpscript_macros::HasSpan;
use miette::SourceSpan;

use crate::dpscript::{
    ast::{ast::Scope, node::Node},
    data::NodeInfo,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct BinaryOpNode {
    pub span: SourceSpan,
    pub operation: BinaryOperation,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
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

impl NodeInfo for BinaryOpNode {
    fn is_const(&self, scope: &Scope) -> bool {
        self.lhs.is_const(scope) && self.rhs.is_const(scope)
    }
}
