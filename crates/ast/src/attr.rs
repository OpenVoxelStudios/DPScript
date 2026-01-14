use crate::{
    data::{SourceSpan, Spanned},
    node::Node,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct AttrNode<'a> {
    pub span: SourceSpan,
    pub name: Spanned<&'a str>,
    pub values: Vec<Node<'a>>,
}
