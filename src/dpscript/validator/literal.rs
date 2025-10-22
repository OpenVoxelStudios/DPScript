use crate::dpscript::{
    ast::literal::{LiteralData, LiteralNode},
    validator::{Result, Validator},
};

impl Validator {
    pub fn validate_literal(&mut self, node: &mut LiteralNode) -> Result<()> {
        match &mut node.data {
            LiteralData::String(_)
            | LiteralData::Int(_)
            | LiteralData::Float(_)
            | LiteralData::Double(_)
            | LiteralData::Bool(_) => (),

            LiteralData::Array(nodes) => {
                // TODO: Do we need to check types here? I think that's handled in the lexer or during
                // computation of the full array type anyway, but I could be wrong.

                for node in nodes {
                    self.validate(node)?;
                }
            }

            LiteralData::Nbt(_nbt_value) => {} // TODO: Validate the schema?
        }

        Ok(())
    }
}
