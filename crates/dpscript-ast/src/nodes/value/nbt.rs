use crate::{prelude::value::Value, util::Name};
use dpscript_core::SourceSpan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct NbtLiteral<'a> {
    pub span: SourceSpan,
    pub values: Vec<(Name<'a>, Value<'a>)>,
}
