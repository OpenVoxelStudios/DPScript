use crate::prelude::{SourceSpan, expr::Expr, scope::Scope, value::Value};
use derivative::Derivative;

#[derive(Debug, Clone, Serialize, Facet, HasSpan, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct At<'a> {
    /// The position argument, the location to run the commands at.
    pub arg: Value<'a>,

    /// The block's body.
    pub body: Vec<Expr<'a>>,

    /// The block's span.
    pub span: SourceSpan,

    #[facet(opaque)]
    #[derivative(PartialEq = "ignore")]
    pub scope: Option<Scope<'a>>,
}
