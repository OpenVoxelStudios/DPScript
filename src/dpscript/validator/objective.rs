use crate::dpscript::{
    ast::objective::ObjectiveNode,
    validator::{Result, Validator},
};
use std::collections::BTreeMap;

impl Validator {
    pub fn validate_objectives(&mut self) -> Result<()> {
        let mut out = BTreeMap::new();

        for (k, mut node) in self.ast.scope.objectives.clone() {
            self.validate_objective(&mut node)?;
            out.insert(k, node);
        }

        for (k, mut node) in self.scope()?.objectives.clone() {
            self.validate_objective(&mut node)?;
            out.insert(k, node);
        }

        self.scope_mut()?.objectives = out;

        Ok(())
    }

    pub fn validate_objective(&mut self, node: &mut ObjectiveNode) -> Result<()> {
        self.validate_ident((&node.name, node.span))?; // TODO: Better span so it's actually the name

        self.scope_mut()?
            .objectives
            .insert(node.name.clone(), node.clone());

        Ok(())
    }
}
