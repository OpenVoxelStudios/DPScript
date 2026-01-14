use crate::{
    attr::AttrNode,
    data::{SourceSpan, Spanned},
    node::Node,
    util::{Body, Indent},
};
use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct BlockNode<'a> {
    pub span: SourceSpan,
    pub body: Vec<Node<'a>>,
    pub kind: BlockKind,
    pub attrs: BTreeMap<Spanned<&'a str>, AttrNode<'a>>,

    /// Whether to exclude this node from dead code elimination.
    pub keep: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BlockKind {
    None,
    Init,
    Tick,
}

impl Display for BlockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockKind::None => write!(f, "none"),
            BlockKind::Init => write!(f, "init"),
            BlockKind::Tick => write!(f, "tick"),
        }
    }
}

impl<'a> Display for BlockNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keep = if self.keep { "[keep] " } else { "" };

        write!(
            f,
            "{keep}block[{}] {{\n{}\n}}",
            self.kind,
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
