use dpscript_core::SourceSpan;

use crate::prelude::value::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct BinOp<'a> {
    pub lhs: Box<Value<'a>>,
    pub op: Operation,
    pub rhs: Box<Value<'a>>,
    pub span: SourceSpan,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet,
)]
pub enum Operation {
    Bool(BoolOp),
    Math(MathOp),
    ArrayIndex,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet,
)]
pub enum BoolOp {
    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet,
)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
