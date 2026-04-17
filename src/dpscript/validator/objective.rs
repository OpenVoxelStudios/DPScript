use ast::objective::ObjectiveNode;

use crate::dpscript::validator::{Result, Validator};
use std::{collections::BTreeMap, rc::Rc};

impl<'a> Validator<'a> {
    pub fn validate_objectives(&mut self) -> Result<()> {
        let mut out = BTreeMap::new();
        let obj = self.ast.borrow().scope.borrow().objectives.clone();

        for (k, mut node) in obj {
            self.validate_objective_ref(&mut node)?;
            out.insert(k, node);
        }

        let obj = self.scope()?.borrow().objectives.clone();

        for (k, mut node) in obj {
            self.validate_objective_ref(&mut node)?;
            out.insert(k, node);
        }

        self.scope()?.borrow_mut().objectives = out;

        Ok(())
    }

    pub fn validate_objective(&mut self, node: &mut ObjectiveNode<'a>) -> Result<()> {
        self.validate_ident(node.name)?; // TODO: Better span so it's actually the name

        self.scope()?
            .borrow_mut()
            .objectives
            .insert(node.name.0, Rc::new(node.clone()));

        Ok(())
    }

    pub fn validate_objective_ref(&mut self, node: &mut Rc<ObjectiveNode<'a>>) -> Result<()> {
        self.validate_ident(node.name)?; // TODO: Better span so it's actually the name

        self.scope()?
            .borrow_mut()
            .objectives
            .insert(node.name.0, Rc::clone(node));

        Ok(())
    }
}
