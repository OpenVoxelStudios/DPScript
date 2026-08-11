use crate::prelude::{SourceSpan, expr::Expr, scope::Scope, value::Value};
use derivative::Derivative;

#[derive(Debug, Clone, Serialize, Facet, HasSpan, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct Cond<'a> {
    /// All the "if" and "if else" conditions in this block.
    pub conditions: Vec<Condition<'a>>,

    /// The nodes in the "else" block (if it exists).
    pub else_block: Vec<Expr<'a>>,

    #[facet(opaque)]
    #[derivative(PartialEq = "ignore")]
    pub else_scope: Option<Scope<'a>>,

    /// The block's span.
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Serialize, Facet, HasSpan, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct Condition<'a> {
    /// The condition for the block.
    pub condition: Value<'a>,

    /// The block's body.
    pub body: Vec<Expr<'a>>,

    /// The block's span.
    pub span: SourceSpan,

    #[facet(opaque)]
    #[derivative(PartialEq = "ignore")]
    pub scope: Option<Scope<'a>>,
}
