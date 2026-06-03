use crate::prelude::{SourceSpan, value::Value};
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Literal<'a> {
    pub value: LiteralValue<'a>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct DslLiteral<'a> {
    pub dsl_marker: DslMarker,
    pub value: Box<Value<'a>>,
    pub span: SourceSpan,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, Deserialize)]
pub enum LiteralValue<'a> {
    String(&'a str),
    Bool(bool),
    Byte(i8),
    Int(i32),
    Long(i64),
    Float(OrderedFloat<f32>),
    Double(OrderedFloat<f64>),
    CurPos,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, Deserialize,
)]
pub enum DslMarker {
    At,   // @"..."
    Hash, // #"..."
}
