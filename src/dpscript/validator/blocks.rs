use std::{cell::RefCell, rc::Rc};

use ast::{
    block::{BlockKind, BlockNode},
    scope::Scope,
};

use crate::dpscript::validator::{Result, Validator, err::Err};

impl<'a> Validator<'a> {
    pub fn validate_blocks(&mut self) -> Result<()> {
        let mut out = Vec::new();
        let blk = self.ast.borrow().scope.borrow().blocks.clone();

        for mut node in blk {
            self.validate_block(&mut node)?;
            out.push(node);
        }

        self.ast.borrow().scope.borrow_mut().blocks = out;

        Ok(())
    }

    pub fn validate_block(&mut self, node: &mut BlockNode<'a>) -> Result<()> {
        if node.kind == BlockKind::None {
            self.errors.push(Err::UntypedBlock {
                span: node.span.into(),
            });
        }

        debug!("Pushing scope (block): {}", node.ident);

        self.scopes.push(Rc::new(RefCell::new(Scope::new(
            &node.ident.path,
            self.scopes.clone(),
        ))));

        for item in &mut node.body {
            self.validate(item)?;
        }

        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        Ok(())
    }
}
