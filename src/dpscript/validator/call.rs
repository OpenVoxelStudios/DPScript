use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::call::CallNode,
        data::NodeInfo,
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_call(&mut self, node: &mut CallNode) -> Result<()> {
        let Some(target) = node.target_fn(self.scope()?) else {
            self.errors.push(Err::UnresolvedRef {
                span: node.span(),
                name: node.func.clone(),
            });

            return Ok(());
        };

        if target.args.len() != node.args.len()
            && (target.args.len() != node.args.len() + 1 && target.receiver.is_some())
        {
            self.errors.push(Err::ArgCountMismatch {
                span: node.span(),
                expected: if target.receiver.is_some() {
                    target.args.len() - 1
                } else {
                    target.args.len()
                },
                got: node.args.len(),
            });

            return Ok(());
        }

        let offset = if target.receiver.is_some() { 1 } else { 0 };
        let real_args = target.args.clone();

        for (i, arg) in node.args.iter_mut().enumerate() {
            self.validate(arg)?;

            let Some(ty) = arg.returns(self.scope()?) else {
                self.errors
                    .push(Err::CannotComputeType { span: arg.span() });

                return Ok(());
            };

            let real = &real_args[i + offset];

            if ty != real.ty {
                self.errors.push(Err::ArgTypeMismatch {
                    span: arg.span(),
                    expected: real.ty.clone(),
                    got: ty,
                });
            }
        }

        Ok(())
    }
}
