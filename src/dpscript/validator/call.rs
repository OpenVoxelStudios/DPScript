use crate::dpscript::validator::{Result, Validator, err::Err};
use ast::{call::CallNode, data::HasSpan};

impl<'a> Validator<'a> {
    pub fn validate_call(&mut self, node: &mut CallNode<'a>) -> Result<()> {
        let Some(target) = node.target_fn(self.scope()?) else {
            self.errors.push(Err::UnresolvedRef {
                span: node.span().into(),
                name: node.func.clone(),
            });

            return Ok(());
        };

        if target.args.len() != node.args.len()
            && (target.args.len() != node.args.len() + 1 && target.receiver.is_some())
        {
            self.errors.push(Err::ArgCountMismatch {
                span: node.span().into(),
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

            // TODO: Type checking

            // let Some(ty) = arg.returns(self.scope()?) else {
            //     self.errors.push(Err::CannotComputeType {
            //         span: arg.span().into(),
            //     });

            //     return Ok(());
            // };

            // let real = &real_args[i + offset];

            // if ty != real.ty {
            //     self.errors.push(Err::ArgTypeMismatch {
            //         span: arg.span().into(),
            //         expected: real.ty.clone(),
            //         got: ty,
            //     });
            // }
        }

        Ok(())
    }
}
