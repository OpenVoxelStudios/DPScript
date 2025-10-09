use std::fmt;

use miette::SourceSpan;

use crate::dpscript::{
    ast::{
        node::Node,
        util::{Body, Indent},
    },
    data::NodeInfo,
};

use super::ast::Scope;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct AtNode {
    pub span: SourceSpan,
    pub pos: Box<Node>,
    pub body: Vec<Node>,
    pub scope: Option<Scope>,
}

impl NodeInfo for AtNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }
}

impl fmt::Display for AtNode {
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
