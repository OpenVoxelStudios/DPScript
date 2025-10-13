use std::collections::BTreeMap;

use crate::dpscript::{
    ast::{
        ast::Scope,
        func::{FuncFlags, FunctionNode},
    },
    validator::{Result, Validator, err::Err},
};

impl Validator {
    pub fn validate_funcs(&mut self) -> Result<()> {
        let mut out = BTreeMap::new();

        for (k, mut node) in self.ast.scope.functions.clone() {
            self.validate_func(&mut node)?;
            out.insert(k, node);
        }

        self.ast.scope.functions = out;

        Ok(())
    }

    pub fn validate_func(&mut self, node: &mut FunctionNode) -> Result<()> {
        self.validate_ident(&(node.name.clone(), node.span))?;

        for arg in &node.args {
            self.validate_ident(&(arg.name.clone(), arg.span))?;
        }

        if node.flags.contains(FuncFlags::Facade) || node.flags.contains(FuncFlags::Compiler) {
            if !node.body.is_empty() {
                self.errors.push(Err::UnexpectedBody { span: node.span });
            }
        }

        self.scopes.push(Scope::default());

        for item in &mut node.body {
            self.validate(item)?;
        }

        node.scope = self.scopes.pop();

        Ok(())
    }
}
