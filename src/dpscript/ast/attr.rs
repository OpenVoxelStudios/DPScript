use miette::SourceSpan;
use crate::dpscript::ast::node::Node;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct AttrNode {
    pub span: SourceSpan,
    pub name: String,
    pub values: Vec<Node>,
}
