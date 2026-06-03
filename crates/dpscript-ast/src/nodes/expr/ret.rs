use crate::prelude::{SourceSpan, value::Value};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct Return<'a> {
    /// An optional value to return.
    pub value: Option<Value<'a>>,

    /// The return statement's span.
    pub span: SourceSpan,
}
