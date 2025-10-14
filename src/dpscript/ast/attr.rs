use crate::dpscript::ast::node::Node;
use flexstr::SharedStr;
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct AttrNode {
    pub span: SourceSpan,
    pub name: SharedStr,
    pub values: Vec<Node>,
}
