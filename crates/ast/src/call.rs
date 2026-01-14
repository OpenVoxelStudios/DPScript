use crate::{data::SourceSpan, node::Node};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct CallNode<'a> {
    pub span: SourceSpan,

    /// A reference to a node that is the receiver,
    /// like when calling an object instance function.
    ///
    /// This will either be a literal identifier or a
    /// [`RefNode`] pointing to one.
    pub receiver: Box<Node<'a>>,

    /// The arguments the function was called with.
    pub args: Vec<Node<'a>>,
}

impl<'a> fmt::Display for CallNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "call {}: [{}]",
            self.receiver,
            self.args
                .iter()
                .map(|it| format!("{it}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
