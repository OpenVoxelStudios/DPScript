use crate::dpscript::ast::node::Node;
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct AttrNode {
    pub span: SourceSpan,
    pub name: String,
    pub values: Vec<Node>,
}
