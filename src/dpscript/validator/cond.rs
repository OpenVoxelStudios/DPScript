use crate::dpscript::validator::{Result, Validator};
use ast::{cond::ConditionalNode, scope::Scope};
use std::{cell::RefCell, rc::Rc};

impl<'a> Validator<'a> {
    pub fn validate_cond(&mut self, node: &mut ConditionalNode<'a>) -> Result<()> {
        self.validate(&mut node.condition)?;

        // TODO: Type checking

        // let Some(cond_ty) = node.condition.returns(self.scope()?) else {
        //     self.errors.push(Err::CannotComputeType {
        //         span: node.condition.span().into(),
        //     });

        //     return Ok(());
        // };

        // if cond_ty != TypeRef::BuiltIn(BuiltInType::Boolean) {
        //     self.errors.push(Err::CondNotBool {
        //         span: node.condition.span().into(),
        //     });
        // }

        debug!("Pushing scope (cond): {}", node.ident);

        self.scopes.push(Rc::new(RefCell::new(Scope::new(
            node.ident.path,
            self.scopes.clone(),
        ))));

        for node in &mut node.body {
            self.validate(node)?;
        }

        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        for node in &mut node.else_ifs {
            self.validate(&mut node.condition)?;

            // TODO: Type checking

            // let Some(cond_ty) = node.condition.returns(self.scope()?) else {
            //     self.errors.push(Err::CannotComputeType {
            //         span: node.condition.span().into(),
            //     });

            //     return Ok(());
            // };

            // if cond_ty != TypeRef::BuiltIn(BuiltInType::Boolean) {
            //     self.errors.push(Err::CondNotBool {
            //         span: node.condition.span().into(),
            //     });
            // }

            debug!("Pushing scope (cond/elseif): {}", node.ident);

            self.scopes.push(Rc::new(RefCell::new(Scope::new(
                &node.ident.path,
                self.scopes.clone(),
            ))));

            for node in &mut node.body {
                self.validate(node)?;
            }

            node.scope = self.scopes.pop();

            debug!("Popped scope!");
        }

        debug!("Pushing scope (cond/else): {}", node.ident);

        self.scopes.push(Rc::new(RefCell::new(Scope::new(
            &node.ident.path,
            self.scopes.clone(),
        ))));

        for node in &mut node.else_body {
            self.validate(node)?;
        }

        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        Ok(())
    }
}
