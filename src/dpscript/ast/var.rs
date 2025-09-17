use crate::{
    dpscript::{
        ast::{ast::Scope, node::Node},
        data::NodeInfo,
        ty::TypeRef,
    },
    util::DataLocation,
};
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct VarNode {
    pub span: SourceSpan,
    pub name: String,
    pub ty: Option<TypeRef>,
    pub value: Option<Box<Node>>,
    pub location: DataLocation,
}

pub trait VarInfo: NodeInfo {
    fn compute_ty(&self, scope: &Scope) -> Option<TypeRef>;
    fn is_const_var(&self) -> bool;
}

impl VarInfo for VarNode {
    fn compute_ty(&self, scope: &Scope) -> Option<TypeRef> {
        match &self.ty {
            Some(ty) => Some(ty.clone()),
            None => self.value.as_ref().map(|it| it.returns(scope)).flatten(),
        }
    }

    fn is_const_var(&self) -> bool {
        false
    }
}

impl NodeInfo for VarNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }
}
