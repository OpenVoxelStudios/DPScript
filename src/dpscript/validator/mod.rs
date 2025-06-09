mod checker;
mod ctx;
mod helpers;

pub use checker::*;
pub use ctx::*;
pub use helpers::*;

use crate::{AST, Module, Result};

#[derive(Debug, Clone)]
pub struct Validator {
    pub ast: AST,
}

impl Validator {
    pub fn new(ast: AST) -> Self {
        Self { ast }
    }

    pub fn run(mut self) -> Result<Self> {
        self.ast.indexed = false;
        self.ast.cached = false;
        self.ast.cache_values()?;

        let mut cx = self.ast.create_checker_context()?;
        let mut modules = cx.modules.clone();

        for (_, module) in &mut modules {
            Module::check(module, &mut cx)?;
        }

        self.ast.modules = Some(modules);

        Ok(self)
    }
}
