use std::collections::BTreeMap;

use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            ast::Scope,
            func::{FuncFlags, FunctionNode},
        },
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_funcs(&mut self) -> Result<()> {
        let fns = self.ast.scope.functions.clone();
        let mut out = BTreeMap::new();

        self.scope_mut()?.functions.extend(fns);

        for (k, mut node) in self.ast.scope.functions.clone() {
            self.validate_func(&mut node)?;
            out.insert(k, node);
        }

        self.scope_mut()?.functions.extend(out.clone());
        self.ast.scope.functions = out;

        Ok(())
    }

    pub fn validate_func(&mut self, node: &mut FunctionNode) -> Result<()> {
        self.validate_ident((&node.name, node.span))?;

        for arg in &node.args {
            self.validate_ident((&arg.name, arg.span))?;
        }

        if node.flags.contains(FuncFlags::Facade) || node.flags.contains(FuncFlags::Compiler) {
            if !node.body.is_empty() {
                self.errors.push(Err::UnexpectedBody { span: node.span });
            }
        }

        debug!("Pushing scope (func): {}", node.ident);

        self.scopes.push(Scope::new(
            format!("{}", node.ident).into(),
            self.scopes.clone(),
        ));

        self.funcs.push(node.clone());

        let mut found_this = false;

        for (i, arg) in node.args.iter().enumerate() {
            self.validate_ident((&arg.name, arg.span))?;
            self.scope_mut()?.add_local(arg.name.clone(), arg.to_var());

            if arg.is_this {
                if node.receiver.is_some() {
                    if found_this {
                        self.errors.push(Err::MultipleThisArg { span: arg.span() });
                    } else {
                        found_this = true;
                    }
                } else {
                    self.errors
                        .push(Err::UnexpectedThisArg { span: arg.span() });
                }

                if i != 0 {
                    self.errors.push(Err::ThisNotFirst { span: arg.span() });
                }
            }
        }

        for item in &mut node.body {
            self.validate(item)?;
        }

        self.funcs.pop();
        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        match &node.receiver {
            Some(it) => self
                .scope_mut()?
                .instance_funcs
                .entry(it.clone())
                .or_default()
                .insert(node.name.clone(), node.clone()),

            None => self
                .scope_mut()?
                .functions
                .insert(node.name.clone(), node.clone()),
        };

        Ok(())
    }
}
