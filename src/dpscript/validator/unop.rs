use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::unop::{UnaryOpNode, UnaryOperation},
        data::NodeInfo,
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_unop(&mut self, node: &mut UnaryOpNode) -> Result<()> {
        let Some(ty) = node.value.returns(self.scope()?) else {
            self.errors.push(Err::CannotComputeType {
                span: node.value.span(),
            });

            return Ok(());
        };

        match node.op {
            UnaryOperation::None => {}
            UnaryOperation::Invert => {
                if !ty.is_truthy() {
                    self.errors.push(Err::NegateNonBool {
                        span: node.value.span(),
                    });
                }
            }
            UnaryOperation::Negate
            | UnaryOperation::LocalOffset
            | UnaryOperation::RangeStart
            | UnaryOperation::RangeEnd => {
                if !ty.is_numeric() {
                    self.errors.push(Err::UnaryNonNumeric {
                        span: node.value.span(),
                    });
                }
            }
        }

        Ok(())
    }
}
