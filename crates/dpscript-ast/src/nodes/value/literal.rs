use crate::prelude::SourceSpan;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Literal<'a> {
    pub value: LiteralValue<'a>,
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
}
