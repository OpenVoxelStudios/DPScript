use crate::dpscript::{ast::{
    constant::ConstantNode, func::FunctionNode, var::{VarInfo, VarNode}
}, ty::TypeRef};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AST {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// A map of globals to their value types.
    /// These globals aren't things like functions, only variables.
    pub globals: HashMap<String, ConstantNode>,

    /// A map of function names to nodes (which are their definitions).
    pub funcs: HashMap<String, FunctionNode>,

    /// A map of types to user-defined instance methods.
    pub instance_funcs: HashMap<TypeRef, HashMap<String, FunctionNode>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    pub module: Module,

    /// A map of local variables to their value types.
    pub locals: HashMap<String, VarNode>,
}

impl Scope {
    pub fn lookup(&self, var: impl AsRef<str>) -> Option<&dyn VarInfo> {
        match self.locals.get(&var.as_ref().to_string()) {
            Some(it) => Some(it),
            None => self
                .module
                .globals
                .get(&var.as_ref().to_string())
                .map(|it| it as &dyn VarInfo),
        }
    }
}
