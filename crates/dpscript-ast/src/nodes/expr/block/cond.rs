use crate::prelude::{SourceSpan, expr::Expr, scope::Scope, value::Value};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct Cond<'a> {
    /// All the "if" and "if else" conditions in this block.
    pub conditions: Vec<Condition<'a>>,

    /// The nodes in the "else" block (if it exists).
    pub else_block: Vec<Expr<'a>>,

    pub else_scope: Option<Scope<'a>>,

    /// The block's span.
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct Condition<'a> {
    /// The condition for the block.
    pub condition: Value<'a>,

    /// The block's body.
    pub body: Vec<Expr<'a>>,

    /// The block's span.
    pub span: SourceSpan,

    pub scope: Option<Scope<'a>>,
}
