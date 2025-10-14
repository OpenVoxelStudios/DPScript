use crate::dpscript::{
    ast::var::VarNode,
    validator::{Result, Validator},
};

impl Validator {
    pub fn validate_variable(&mut self, node: &VarNode) -> Result<()> {
        self.validate_ident(&(node.name.clone(), node.span))?; // TODO: Improve the span so it's actually just the name

        // TODO: types & values

        self.scope_mut()?.add_local(node.name.clone(), node.clone());

        Ok(())
    }
}
