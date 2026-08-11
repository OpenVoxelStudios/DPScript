use derivative::Derivative;

use crate::{
    prelude::{SourceSpan, expr::Expr, scope::Scope, value::Value},
    util::Name,
};

#[derive(Debug, Clone, Serialize, Facet, HasSpan, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct ForLoop<'a> {
    /// The value to iterate through.
    pub array: Value<'a>,

    /// The name of the index/object variable inside the loop.
    pub var: Name<'a>,

    /// The block's body.
    pub body: Vec<Expr<'a>>,

    /// The block's span.
    pub span: SourceSpan,

    #[facet(opaque)]
    #[derivative(PartialEq = "ignore")]
    pub scope: Option<Scope<'a>>,
}
