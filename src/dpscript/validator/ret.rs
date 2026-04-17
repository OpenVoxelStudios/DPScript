use ast::ret::ReturnNode;
use crate::dpscript::validator::{Result, Validator};

impl<'a> Validator<'a> {
    pub fn validate_return(&mut self, _node: &mut ReturnNode<'a>) -> Result<()> {
        // TODO: Type checking

        // let func = self.func()?;

        // let Some(value) = &node.value else {
        //     if func.return_type != TypeRef::BuiltIn(BuiltInType::Void) {
        //         self.errors.push(Err::MustReturnValue {
        //             span: node.span().into(),
        //         });
        //     }

        //     return Ok(());
        // };

        // let Some(ty) = value.returns(self.scope()?) else {
        //     self.errors.push(Err::CannotComputeType {
        //         span: value.span().into(),
        //     });

        //     return Ok(());
        // };

        // if ty != func.return_type {
        //     self.errors.push(Err::ReturnTypeMismatch {
        //         span: value.span().into(),
        //         expected: func.return_type,
        //         got: ty,
        //     });
        // }

        Ok(())
    }
}
