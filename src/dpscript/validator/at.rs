use crate::dpscript::validator::{Result, Validator};
use ast::{at::AtNode, scope::Scope};
use std::{cell::RefCell, rc::Rc};

impl<'a> Validator<'a> {
    pub fn validate_at(&mut self, node: &mut AtNode<'a>) -> Result<()> {
        // TODO: Type checking

        // let Some(ty) = node.pos.returns(self.scope()?) else {
        //     self.errors.push(Err::CannotComputeType {
        //         span: node.pos.span().into(),
        //     });

        //     return Ok(());
        // };

        // if ty != TypeRef::BuiltIn(BuiltInType::Pos) {
        //     self.errors.push(Err::AtNotPos {
        //         span: node.pos.span().into(),
        //     });
        // }

        debug!("Pushing scope (at): {}", node.ident);

        self.scopes.push(Rc::new(RefCell::new(Scope::new(
            &node.ident.path,
            self.scopes.clone(),
        ))));

        for node in &mut node.body {
            self.validate(node)?;
        }

        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        Ok(())
    }
}
