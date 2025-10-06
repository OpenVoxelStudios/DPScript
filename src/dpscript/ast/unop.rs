//! Unary operations

use std::fmt;

use miette::SourceSpan;

use crate::dpscript::{
    ast::{ast::Scope, node::Node},
    data::NodeInfo,
    ty::TypeRef,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct UnaryOpNode {
    pub span: SourceSpan,
    pub op: UnaryOperation,
    pub value: Box<Node>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum UnaryOperation {
    /// value
    None,

    /// !value
    Invert,

    /// ~value
    BitNot,

    /// -value
    Negate,

    // value..
    RangeStart,

    /// ..value
    RangeEnd,
}

impl NodeInfo for UnaryOpNode {
    fn is_const(&self, scope: &Scope) -> bool {
        self.value.is_const(scope)
    }

    fn returns(&self, scope: &Scope) -> Option<TypeRef> {
        self.value.returns(scope)
    }
}

impl fmt::Display for UnaryOpNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(feature = "print-clarity"))]
        match self.op {
            UnaryOperation::None => write!(f, "{}", self.value),
            UnaryOperation::Invert => write!(f, "-{}", self.value),
            UnaryOperation::BitNot => write!(f, "~{}", self.value),
            UnaryOperation::Negate => write!(f, "!{}", self.value),
            UnaryOperation::RangeStart => write!(f, "{}..", self.value),
            UnaryOperation::RangeEnd => write!(f, "..{}", self.value),
        }

        #[cfg(feature = "print-clarity")]
        match self.op {
            UnaryOperation::None => write!(f, "unary<None, {}>", self.value),
            UnaryOperation::Invert => write!(f, "unary<Invert, {}>", self.value),
            UnaryOperation::BitNot => write!(f, "unary<BitwiseNot, {}>", self.value),
            UnaryOperation::Negate => write!(f, "unary<Negate, {}>", self.value),
            UnaryOperation::RangeStart => write!(f, "unary<RangeStart, {}>", self.value),
            UnaryOperation::RangeEnd => write!(f, "unary<RangeEnd, {}>", self.value),
        }
    }
}
