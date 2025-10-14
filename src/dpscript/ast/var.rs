use std::fmt;

use crate::{
    dpscript::{
        ast::{ast::Scope, node::Node},
        data::NodeInfo,
        ty::TypeRef,
    },
    util::DataLocation,
};
use flexstr::SharedStr;
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct VarNode {
    pub span: SourceSpan,
    pub name: SharedStr,
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

impl fmt::Display for VarNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = if let Some(ty) = &self.ty {
            format!(" [type: {ty}]")
        } else {
            "".into()
        };

        let val = if let Some(val) = &self.value {
            format!(" = {val}")
        } else {
            "".into()
        };

        write!(f, "var {} @ [{}]{ty}{val};", self.name, self.location)
    }
}
