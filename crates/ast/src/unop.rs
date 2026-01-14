//! Unary operations

use crate::{data::SourceSpan, node::Node};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct UnaryOpNode<'a> {
    pub span: SourceSpan,
    pub op: UnaryOperation,
    pub value: Box<Node<'a>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum UnaryOperation {
    /// value
    None,

    /// !value
    Invert,

    // FIXME: I don't really like that you have to do `~0` in order to just
    // get `~` in Minecraft, but it's necessary for the way we parse things
    // right now. Fix this!
    /// ~value
    LocalOffset,

    /// -value
    Negate,

    // value..
    RangeStart,

    /// ..value
    RangeEnd,
}

impl<'a> fmt::Display for UnaryOpNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(feature = "print-clarity"))]
        match self.op {
            UnaryOperation::None => write!(f, "{}", self.value),
            UnaryOperation::Invert => write!(f, "!{}", self.value),
            UnaryOperation::LocalOffset => write!(f, "~{}", self.value),
            UnaryOperation::Negate => write!(f, "-{}", self.value),
            UnaryOperation::RangeStart => write!(f, "{}..", self.value),
            UnaryOperation::RangeEnd => write!(f, "..{}", self.value),
        }

        #[cfg(feature = "print-clarity")]
        match self.op {
            UnaryOperation::None => write!(f, "unary<None, {}>", self.value),
            UnaryOperation::Invert => write!(f, "unary<Invert, {}>", self.value),
            UnaryOperation::LocalOffset => write!(f, "unary<LocalOffset, {}>", self.value),
            UnaryOperation::Negate => write!(f, "unary<Negate, {}>", self.value),
            UnaryOperation::RangeStart => write!(f, "unary<RangeStart, {}>", self.value),
            UnaryOperation::RangeEnd => write!(f, "unary<RangeEnd, {}>", self.value),
        }
    }
}
