use ast::enums::EnumNode;
use crate::dpscript::validator::{Result, Validator};

impl<'a> Validator<'a> {
    pub fn validate_enum(&mut self, node: &mut EnumNode<'a>) -> Result<()> {
        self.validate_ident(node.name)?;

        for id in &node.values {
            self.validate_ident(*id)?;
        }

        Ok(())
    }
}
