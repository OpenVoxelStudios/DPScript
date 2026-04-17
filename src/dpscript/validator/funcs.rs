use crate::dpscript::validator::{Result, Validator, err::Err};
use ast::{
    data::HasSpan,
    func::{FuncFlags, FunctionNode},
    scope::Scope,
};
use std::{cell::RefCell, rc::Rc};

impl<'a> Validator<'a> {
    pub fn validate_funcs(&mut self) -> Result<()> {
        let mut fns = self.ast.borrow().scope.borrow_mut().functions.clone();

        self.scope()?.borrow_mut().functions.extend(fns.clone());

        for (_, node) in &mut fns {
            let copy = FunctionNode::clone(node);
            let copy = self.validate_func(copy)?;
            *node = copy;
        }

        self.ast.borrow().scope.borrow_mut().functions = fns;

        let mut fns = self.ast.borrow().scope.borrow_mut().instance_funcs.clone();

        self.scope()?
            .borrow_mut()
            .instance_funcs
            .extend(fns.clone());

        for (_, map) in &mut fns {
            for (_, node) in map {
                let copy = FunctionNode::clone(node);
                let copy = self.validate_func(copy)?;
                *node = copy;
            }
        }

        self.ast.borrow().scope.borrow_mut().instance_funcs = fns;

        Ok(())
    }

    pub fn validate_func(&mut self, mut node: FunctionNode<'a>) -> Result<Rc<FunctionNode<'a>>> {
        self.validate_ident(node.name)?;

        for arg in &node.args {
            self.validate_ident(arg.name)?;
        }

        if node.flags.contains(FuncFlags::Facade) || node.flags.contains(FuncFlags::Compiler) {
            if !node.body.is_empty() {
                self.errors.push(Err::UnexpectedBody {
                    span: node.span.into(),
                });
            }
        }

        debug!("Pushing scope (func): {}", node.ident);

        self.scopes.push(Rc::new(RefCell::new(Scope::new(
            &node.ident.path,
            self.scopes.clone(),
        ))));

        self.funcs.push(node.clone());

        let mut found_this = false;

        for (i, arg) in node.args.iter().enumerate() {
            self.validate_ident(arg.name)?;

            self.scope()?
                .borrow_mut()
                .add_local(arg.name.0, arg.to_var());

            if arg.is_this {
                if node.receiver.is_some() {
                    if found_this {
                        self.errors.push(Err::MultipleThisArg {
                            span: arg.span().into(),
                        });
                    } else {
                        found_this = true;
                    }
                } else {
                    self.errors.push(Err::UnexpectedThisArg {
                        span: arg.span().into(),
                    });
                }

                if i != 0 {
                    self.errors.push(Err::ThisNotFirst {
                        span: arg.span().into(),
                    });
                }
            }
        }

        for item in &mut node.body {
            self.validate(item)?;
        }

        self.funcs.pop();
        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        let ref_ = Rc::new(node);

        match &node.receiver {
            Some(it) => self
                .scope()?
                .borrow_mut()
                .instance_funcs
                .entry(it.0)
                .or_default()
                .insert(node.name.0, ref_),

            None => self
                .scope()?
                .borrow_mut()
                .functions
                .insert(node.name.0, ref_),
        };

        Ok(ref_)
    }
}
