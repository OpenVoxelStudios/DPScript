use std::collections::HashMap;

use crate::dpscript::ast::ast::{Module, AST};

pub struct Validator {
    /// The merged AST.
    pub ast: AST,

    /// A map of paths to modules.
    pub module_map: HashMap<String, Module>,
}

impl Validator {
    pub fn new(ast: AST) -> Self {
        Self {
            ast,
            module_map: todo!(),
        }
    }
}
