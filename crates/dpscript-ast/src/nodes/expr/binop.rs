//! Binary operations

use crate::{
    data::{SourceSpan, Spanned},
    node::Node,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct BinaryOpNode<'a> {
    pub span: SourceSpan,
    pub op: Spanned<BinaryOperation>,
    pub lhs: Box<Node<'a>>,
    pub rhs: Box<Node<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BinaryOperation {
    /// lhs + rhs
    Add,

    /// lhs - rhs
    Sub,

    /// lhs * rhs
    Mul,

    /// lhs / rhs
    Div,

    /// lhs % rhs
    Mod,

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

    /// lhs = rhs
    Assign,

    /// lhs += rhs
    AddAssign,

    /// lhs -= rhs
    SubAssign,

    /// lhs *= rhs
    MulAssign,

    /// lhs /= rhs
    DivAssign,

    /// lhs %= rhs
    ModAssign,

    /// lhs &= rhs
    BitAndAssign,

    /// lhs |= rhs
    BitOrAssign,

    /// lhs ^= rhs
    BitXorAssign,

    /// lhs .. rhs
    Range,

    /// lhs.rhs
    Field,

    /// lhs[rhs]
    ArrayIndex,
}

impl<'a> fmt::Display for BinaryOpNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(feature = "print-clarity"))]
        match self.op.0 {
            BinaryOperation::Add => write!(f, "{} + {}", self.lhs, self.rhs),
            BinaryOperation::Sub => write!(f, "{} - {}", self.lhs, self.rhs),
            BinaryOperation::Mul => write!(f, "{} * {}", self.lhs, self.rhs),
            BinaryOperation::Div => write!(f, "{} / {}", self.lhs, self.rhs),
            BinaryOperation::Mod => write!(f, "{} % {}", self.lhs, self.rhs),
            BinaryOperation::BitAnd => write!(f, "{} & {}", self.lhs, self.rhs),
            BinaryOperation::BitOr => write!(f, "{} | {}", self.lhs, self.rhs),
            BinaryOperation::BitXor => write!(f, "{} ^ {}", self.lhs, self.rhs),
            BinaryOperation::CondAnd => write!(f, "{} && {}", self.lhs, self.rhs),
            BinaryOperation::CondOr => write!(f, "{} || {}", self.lhs, self.rhs),
            BinaryOperation::CondEq => write!(f, "{} == {}", self.lhs, self.rhs),
            BinaryOperation::CondNeq => write!(f, "{} != {}", self.lhs, self.rhs),
            BinaryOperation::CondGt => write!(f, "{} > {}", self.lhs, self.rhs),
            BinaryOperation::CondGe => write!(f, "{} >= {}", self.lhs, self.rhs),
            BinaryOperation::CondLt => write!(f, "{} < {}", self.lhs, self.rhs),
            BinaryOperation::CondLe => write!(f, "{} <= {}", self.lhs, self.rhs),
            BinaryOperation::Assign => write!(f, "{} = {};", self.lhs, self.rhs),
            BinaryOperation::AddAssign => write!(f, "{} += {};", self.lhs, self.rhs),
            BinaryOperation::SubAssign => write!(f, "{} -= {};", self.lhs, self.rhs),
            BinaryOperation::MulAssign => write!(f, "{} *= {};", self.lhs, self.rhs),
            BinaryOperation::DivAssign => write!(f, "{} /= {};", self.lhs, self.rhs),
            BinaryOperation::ModAssign => write!(f, "{} %= {};", self.lhs, self.rhs),
            BinaryOperation::BitAndAssign => write!(f, "{} &= {};", self.lhs, self.rhs),
            BinaryOperation::BitOrAssign => write!(f, "{} |= {};", self.lhs, self.rhs),
            BinaryOperation::BitXorAssign => write!(f, "{} ^= {};", self.lhs, self.rhs),
            BinaryOperation::Range => write!(f, "{} .. {}", self.lhs, self.rhs),
            BinaryOperation::Field => write!(f, "{}.{}", self.lhs, self.rhs),
            BinaryOperation::ArrayIndex => write!(f, "{}[{}]", self.lhs, self.rhs),
        }

        #[cfg(feature = "print-clarity")]
        match self.op.0 {
            BinaryOperation::Add => write!(f, "binary<Add, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Sub => write!(f, "binary<Sub, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Mul => write!(f, "binary<Mul, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Div => write!(f, "binary<Div, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Mod => write!(f, "binary<Mod, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::BitAnd => write!(f, "binary<BitAnd, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::BitOr => write!(f, "binary<BitOr, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::BitXor => write!(f, "binary<BitXor, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondAnd => write!(f, "binary<CondAnd, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondOr => write!(f, "binary<CondOr, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondEq => write!(f, "binary<CondEq, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondNeq => write!(f, "binary<CondNeq, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondGt => write!(f, "binary<CondGt, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondGe => write!(f, "binary<CondGe, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondLt => write!(f, "binary<CondLt, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondLe => write!(f, "binary<CondLe, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Assign => write!(f, "binary<Assign, {}, {}>;", self.lhs, self.rhs),
            BinaryOperation::AddAssign => {
                write!(f, "binary<AddAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::SubAssign => {
                write!(f, "binary<SubAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::MulAssign => {
                write!(f, "binary<MulAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::DivAssign => {
                write!(f, "binary<DivAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::ModAssign => {
                write!(f, "binary<ModAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::BitAndAssign => {
                write!(f, "binary<BitAndAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::BitOrAssign => {
                write!(f, "binary<BitOrAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::BitXorAssign => {
                write!(f, "binary<BitXorAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::Range => write!(f, "binary<Range, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Field => write!(f, "binary<Field, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::ArrayIndex => {
                write!(f, "binary<ArrayIndex, {}, {}>", self.lhs, self.rhs)
            }
        }
    }
}
