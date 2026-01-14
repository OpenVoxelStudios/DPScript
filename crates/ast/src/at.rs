use crate::{
    data::SourceSpan,
    node::Node,
    util::{Body, Indent},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct AtNode<'a> {
    pub span: SourceSpan,
    pub pos: Box<Node<'a>>,
    pub body: Vec<Node<'a>>,
}

impl<'a> fmt::Display for AtNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "@at [{}]: {{\n{}\n}};",
            self.pos,
            self.body
                .iter()
                .map(|it| format!("{it}"))
                .collect::<Vec<_>>()
                .join("\n")
                .indent(4)
                .body()
        )
    }
}
