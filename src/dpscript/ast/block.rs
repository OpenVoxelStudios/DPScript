use crate::dpscript::{
    ast::{ast::Scope, node::Node},
    data::NodeInfo,
    ty::TypeRef,
};
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
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

    fn returns(&self, scope: &Scope) -> Option<TypeRef> {
        if let Some(last) = self.body.last() {
            if last.is_end {
                return last.returns(scope);
            }
        }

        None
    }
}
