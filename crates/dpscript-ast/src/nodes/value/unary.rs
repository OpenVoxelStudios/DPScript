use dpscript_core::SourceSpan;

use crate::prelude::value::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Unary<'a> {
    pub value: Box<Value<'a>>,
    pub op: UnaryOp,
    pub span: SourceSpan,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet,
)]
pub enum UnaryOp {
    /// Math operation - negate
    /// `-value`
    Negate,

    /// Boolean operation - invert
    /// `!bool`
    Invert,
}
