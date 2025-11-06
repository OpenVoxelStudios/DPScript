use std::fmt::{self, Display};

use crate::{
    dpscript::{
        ast::{ast::Scope, node::Node},
        data::NodeInfo,
        ty::TypeRef,
    },
    util::DataLocation,
};
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct VarNode {
    pub span: SourceSpan,
    pub name: String,
    pub ty: Option<TypeRef>,
    pub value: Option<Box<Node>>,
    pub location: DataLocation,

    /// Is this variable a function argument?
    /// This shouldn't be set during lexing - only during validation (via [`super::func::FunctionArg::to_var`]).
    pub is_arg: bool,
}

pub trait VarInfo: NodeInfo + Display {
    fn as_node(&self) -> Node;
    fn compute_ty(&self, scope: &Scope) -> Option<TypeRef>;
    fn is_const_var(&self) -> bool;
}

impl VarInfo for VarNode {
    fn as_node(&self) -> Node {
        Node::Variable(self.clone())
    }

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
