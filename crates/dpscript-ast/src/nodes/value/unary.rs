use dpscript_core::SourceSpan;

use crate::{
    prelude::{def::func::FunctionInfo, value::Value},
    util::Remote,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan, Default)]
pub struct Unary<'a> {
    pub value: Box<Value<'a>>,
    pub op: UnaryOp,
    pub span: SourceSpan,
    pub resolved: Option<Remote<FunctionInfo<'a>>>,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet, Default,
)]
pub enum UnaryOp {
    /// Math operation - negate
    /// `-value`
    Negate,

    /// Boolean operation - invert
    /// `!bool`
    Invert,

    /// Offset from the current position.
    /// `~value`
    Offset,

    /// Only used for analysis; should never appear in the final AST.
    #[default]
    None,
}
