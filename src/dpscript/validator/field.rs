use crate::dpscript::{
    ast::field::FieldNode,
    validator::{Result, Validator},
};

impl Validator {
    pub fn validate_field(&mut self, node: &mut FieldNode) -> Result<()> {
        self.validate_ident((&node.name, node.span))?;

        Ok(())
    }
}
