use super::Node;
use crate::Spanned;
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Assign {
    /// The target variable to assign to.
    pub target: Spanned<String>,

    /// The value to assign.
    pub value: Box<Node>,

    /// The span.
    pub span: SourceSpan,
}
