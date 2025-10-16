use crate::dpscript::{
    ast::enums::EnumNode,
    validator::{Result, Validator},
};

impl Validator {
    pub fn validate_enum(&mut self, node: &mut EnumNode) -> Result<()> {
        self.validate_ident((&node.name, node.span))?;

        for id in &node.values {
            self.validate_ident((id, node.span))?;
        }

        Ok(())
    }
}
