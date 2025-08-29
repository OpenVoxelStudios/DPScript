//! Unary operations

use miette::SourceSpan;

use crate::dpscript::{ast::{ast::Scope, node::Node}, data::NodeInfo, ty::TypeRef};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UnaryOpNode {
    pub span: SourceSpan,
    pub operation: UnaryOperation,
    pub value: Box<Node>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum UnaryOperation {
    /// value
    None,

    /// !value
    Negate,

    /// ~value
    BitNot,
}

impl NodeInfo for UnaryOpNode {
    fn is_const(&self, scope: &Scope) -> bool {
        self.value.is_const(scope)
    }

    fn returns(&self, scope: &Scope) -> Option<TypeRef> {
        self.value.returns(scope)
    }
}
