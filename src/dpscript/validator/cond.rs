use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{ast::Scope, cond::ConditionalNode},
        data::NodeInfo,
        ty::{BuiltInType, TypeRef},
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_cond(&mut self, node: &mut ConditionalNode) -> Result<()> {
        self.validate(&mut node.condition)?;

        let Some(cond_ty) = node.condition.returns(self.scope()?) else {
            self.errors.push(Err::CannotComputeType {
                span: node.condition.span(),
            });

            return Ok(());
        };

        if cond_ty != TypeRef::BuiltIn(BuiltInType::Boolean) {
            self.errors.push(Err::CondNotBool {
                span: node.condition.span(),
            });
        }

        debug!("Pushing scope (cond): {}", node.ident);

        self.scopes.push(Scope::new(
            format!("{}", node.ident).into(),
            self.scopes.clone(),
        ));

        for node in &mut node.body {
            self.validate(node)?;
        }

        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        for node in &mut node.else_ifs {
            self.validate(&mut node.condition)?;

            let Some(cond_ty) = node.condition.returns(self.scope()?) else {
                self.errors.push(Err::CannotComputeType {
                    span: node.condition.span(),
                });

                return Ok(());
            };

            if cond_ty != TypeRef::BuiltIn(BuiltInType::Boolean) {
                self.errors.push(Err::CondNotBool {
                    span: node.condition.span(),
                });
            }

            debug!("Pushing scope (cond/elseif): {}", node.ident);

            self.scopes.push(Scope::new(
                format!("{}", node.ident).into(),
                self.scopes.clone(),
            ));

            for node in &mut node.body {
                self.validate(node)?;
            }

            node.scope = self.scopes.pop();

            debug!("Popped scope!");
        }

        debug!("Pushing scope (cond/else): {}", node.ident);

        self.scopes.push(Scope::new(
            format!("{}", node.ident).into(),
            self.scopes.clone(),
        ));

        for node in &mut node.else_body {
            self.validate(node)?;
        }

        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        Ok(())
    }
}
