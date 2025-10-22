use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::ret::ReturnNode,
        data::NodeInfo,
        ty::{BuiltInType, TypeRef},
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_return(&mut self, node: &mut ReturnNode) -> Result<()> {
        let func = self.func()?;

        let Some(value) = &node.value else {
            if func.return_type != TypeRef::BuiltIn(BuiltInType::Void) {
                self.errors.push(Err::MustReturnValue { span: node.span() });
            }

            return Ok(());
        };

        let Some(ty) = value.returns(self.scope()?) else {
            self.errors
                .push(Err::CannotComputeType { span: value.span() });

            return Ok(());
        };

        if ty != func.return_type {
            self.errors.push(Err::ReturnTypeMismatch {
                span: value.span(),
                expected: func.return_type.clone(),
                got: ty,
            });
        }

        Ok(())
    }
}
