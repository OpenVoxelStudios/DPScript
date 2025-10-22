use std::fmt;

use crate::{
    dpscript::{
        ast::{
            ast::Scope,
            node::Node,
            var::{VarInfo, VarNode},
        },
        data::NodeInfo,
        ty::TypeRef,
    },
    util::DataLocation,
};
use dpscript_macros::HasSpan;
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct ConstantNode {
    pub is_public: bool,
    pub span: SourceSpan,
    pub name: String,
    pub ty: Option<TypeRef>,
    pub value: Box<Node>,

    /// Whether to exclude this node from dead code elimination.
    pub keep: bool,
}

impl ConstantNode {
    pub fn as_var(&self) -> VarNode {
        VarNode {
            span: self.span,
            name: self.name.clone(),
            ty: self.ty.clone(),
            value: Some(self.value.clone()),
            location: DataLocation {
                storage: "dpscript:__internal".into(),
                path: "__compiler_internal".into(), // Constants aren't available at runtime so this is a dummy location
            },
        }
    }
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

        let keep = if self.keep { "[keep] " } else { "" };
        let public = if self.is_public { "[public] " } else { "" };

        write!(f, "{keep}const {public}{}{ty} = {};", self.name, self.value)
    }
}
