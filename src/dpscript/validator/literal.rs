use ast::literal::{LiteralData, LiteralNode};

use crate::dpscript::validator::{Result, Validator};

impl<'a> Validator<'a> {
    pub fn validate_literal(&mut self, node: &mut LiteralNode<'a>) -> Result<()> {
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

            LiteralData::Ident(it) => self.validate_ident_literal((*it, node.span))?,
            LiteralData::Nbt(_nbt_value) => {} // TODO: Validate the schema?
        }

        Ok(())
    }
}
