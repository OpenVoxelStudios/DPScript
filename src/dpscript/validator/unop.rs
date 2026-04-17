use crate::dpscript::validator::{Result, Validator};
use ast::unop::UnaryOpNode;

impl<'a> Validator<'a> {
    pub fn validate_unop(&mut self, _node: &mut UnaryOpNode<'a>) -> Result<()> {
        // TODO: Type checking

        // let Some(ty) = node.value.returns(self.scope()?) else {
        //     self.errors.push(Err::CannotComputeType {
        //         span: node.value.span().into(),
        //     });

        //     return Ok(());
        // };

        // match node.op {
        //     UnaryOperation::None => {}
        //     UnaryOperation::Invert => {
        //         if !ty.is_truthy() {
        //             self.errors.push(Err::NegateNonBool {
        //                 span: node.value.span().into(),
        //             });
        //         }
        //     }
        //     UnaryOperation::Negate
        //     | UnaryOperation::LocalOffset
        //     | UnaryOperation::RangeStart
        //     | UnaryOperation::RangeEnd => {
        //         if !ty.is_numeric() {
        //             self.errors.push(Err::UnaryNonNumeric {
        //                 span: node.value.span().into(),
        //             });
        //         }
        //     }
        // }

        Ok(())
    }
}
