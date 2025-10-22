use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{ast::Scope, at::AtNode},
        data::NodeInfo,
        ty::{BuiltInType, TypeRef},
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_at(&mut self, node: &mut AtNode) -> Result<()> {
        let Some(ty) = node.pos.returns(self.scope()?) else {
            self.errors.push(Err::CannotComputeType {
                span: node.pos.span(),
            });

            return Ok(());
        };

        if ty != TypeRef::BuiltIn(BuiltInType::Pos) {
            self.errors.push(Err::AtNotPos {
                span: node.pos.span(),
            });
        }

        debug!("Pushing scope (at): {}", node.ident);

        self.scopes.push(Scope::new(
            format!("{}", node.ident).into(),
            self.scopes.clone(),
        ));

        for node in &mut node.body {
            self.validate(node)?;
        }

        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        Ok(())
    }
}
