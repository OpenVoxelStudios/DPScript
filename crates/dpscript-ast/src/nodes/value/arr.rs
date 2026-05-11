use crate::prelude::value::Value;
use dpscript_core::SourceSpan;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct ArrayLiteral<'a> {
    pub span: SourceSpan,
    pub values: Vec<Value<'a>>,
}
