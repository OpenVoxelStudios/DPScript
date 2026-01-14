use crate::{
    data::{SourceSpan, Spanned},
    node::Node,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct ConstantNode<'a> {
    pub is_public: bool,
    pub span: SourceSpan,
    pub name: Spanned<&'a str>,
    pub ty: Option<Spanned<&'a str>>,
    pub value: Box<Node<'a>>,

    /// Whether to exclude this node from dead code elimination.
    pub keep: bool,
}

impl<'a> fmt::Display for ConstantNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = match &self.ty {
            Some((it, _)) => format!(" [type: {it}]"),
            None => "".into(),
        };

        let keep = if self.keep { "[keep] " } else { "" };
        let public = if self.is_public { "[public] " } else { "" };

        write!(
            f,
            "{keep}const {public}{}{ty} = {};",
            self.name.0, self.value
        )
    }
}
