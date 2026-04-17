use ast::field::FieldNode;

use crate::dpscript::validator::{Result, Validator};

impl<'a> Validator<'a> {
    pub fn validate_field(&mut self, node: &mut FieldNode<'a>) -> Result<()> {
        self.validate_ident(node.name)?;

        Ok(())
    }
}
