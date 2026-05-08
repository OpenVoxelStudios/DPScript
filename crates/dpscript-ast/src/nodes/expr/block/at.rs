use crate::prelude::{SourceSpan, expr::Expr, value::Value};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct At<'a> {
    /// The position argument, the location to run the commands at.
    pub arg: Value<'a>,

    /// The block's body.
    pub body: Vec<Expr<'a>>,

    /// The block's span.
    pub span: SourceSpan,
}
