use crate::{
    prelude::{SourceSpan, def::func::FunctionInfo, value::Value},
    util::Remote,
};
use ordered_float::OrderedFloat;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan, Default,
)]
pub struct Literal<'a> {
    pub value: LiteralValue<'a>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan, Default)]
pub struct DslLiteral<'a> {
    pub dsl_marker: DslMarker,
    pub value: Box<Value<'a>>,
    pub span: SourceSpan,
    pub resolved: Option<Remote<FunctionInfo<'a>>>,
}

#[repr(u8)]
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, Deserialize, Default,
)]
pub enum LiteralValue<'a> {
    String(&'a str),
    Bool(bool),
    Byte(i8),
    Int(i32),
    Long(i64),
    Float(OrderedFloat<f32>),
    Double(OrderedFloat<f64>),
    CurPos,

    /// This is only for analysis, actual parsing does not support a null type.
    /// This should never appear in the final AST.
    #[default]
    Null,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, Deserialize, Default,
)]
pub enum DslMarker {
    At,   // @"..."
    Hash, // #"..."

    /// This is only for analysis, and should never appear in the final AST.
    #[default]
    None,
}
