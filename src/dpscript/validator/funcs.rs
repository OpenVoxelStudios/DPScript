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
        let mut fns = self.ast.scope.functions.clone();

        self.scope_mut()?.functions.extend(fns.clone());

        for (_, node) in &mut fns {
            self.validate_func(node)?;
        }

        self.ast.scope.functions = fns;

        let mut fns = self.ast.scope.instance_funcs.clone();

        self.scope_mut()?.instance_funcs.extend(fns.clone());

        for (_, map) in &mut fns {
            for (_, node) in map {
                self.validate_func(node)?;
            }
        }

        self.ast.scope.instance_funcs = fns;

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
