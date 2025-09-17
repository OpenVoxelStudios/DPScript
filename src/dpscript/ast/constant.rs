use crate::dpscript::{
    ast::{ast::Scope, node::Node, var::VarInfo},
    data::NodeInfo,
    ty::TypeRef,
};
use dpscript_macros::HasSpan;
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ConstantNode {
    pub span: SourceSpan,
    pub name: String,
    pub ty: TypeRef,
    pub value: Box<Node>,
}

impl VarInfo for ConstantNode {
    fn compute_ty(&self, _scope: &Scope) -> Option<TypeRef> {
        Some(self.ty.clone())
    }

    fn is_const_var(&self) -> bool {
        true
    }
}

impl NodeInfo for ConstantNode {
    // It's a variable declaration and therefore has no value!
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }
}
