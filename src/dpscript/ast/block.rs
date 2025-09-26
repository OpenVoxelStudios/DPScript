use std::fmt::Display;

use crate::dpscript::{
    ast::{ast::Scope, node::Node},
    data::NodeInfo,
    ty::TypeRef,
};
use dpscript_macros::HasSpan;
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct BlockNode {
    pub span: SourceSpan,
    pub body: Vec<Node>,
    pub kind: BlockKind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BlockKind {
    None,
    Init,
    Tick,
}

impl NodeInfo for BlockNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }

    fn returns(&self, _scope: &Scope) -> Option<TypeRef> {
        None
    }
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
