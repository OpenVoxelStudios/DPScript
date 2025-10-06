use std::fmt;

use crate::dpscript::{
    ast::{ast::Scope, node::Node, var::VarInfo},
    data::NodeInfo,
    ty::TypeRef,
};
use dpscript_macros::HasSpan;
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ConstantNode {
    pub is_public: bool,
    pub span: SourceSpan,
    pub name: String,
    pub ty: Option<TypeRef>,
    pub value: Box<Node>,
}

impl VarInfo for ConstantNode {
    fn compute_ty(&self, _scope: &Scope) -> Option<TypeRef> {
        self.ty.clone()
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

impl fmt::Display for ConstantNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = match &self.ty {
            Some(it) => format!(" [type: {it}]"),
            None => "".into(),
        };

        if self.is_public {
            write!(f, "const [public] {}{ty} = {};", self.name, self.value)
        } else {
            write!(f, "const {}{ty} = {};", self.name, self.value)
        }
    }
}
