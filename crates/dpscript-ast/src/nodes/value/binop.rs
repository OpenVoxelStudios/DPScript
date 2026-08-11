use dpscript_core::SourceSpan;

use crate::{
    prelude::{def::func::FunctionInfo, value::Value},
    util::Remote,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan, Default)]
pub struct BinOp<'a> {
    pub lhs: Box<Value<'a>>,
    pub op: Operation,
    pub rhs: Box<Value<'a>>,
    pub span: SourceSpan,
    pub resolved: Option<Remote<FunctionInfo<'a>>>,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet, Default,
)]
pub enum Operation {
    Bool(BoolOp),
    Math(MathOp),
    ArrayIndex,

    /// This is only for analysis, and should never appear in the final AST.
    #[default]
    None,
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
    And,
    Or,
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
