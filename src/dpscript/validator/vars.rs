use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::var::VarNode,
        data::NodeInfo,
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_variable(&mut self, node: &mut VarNode) -> Result<()> {
        self.validate_ident((&node.name, node.span))?; // TODO: Improve the span so it's actually just the name

        // types should be automatically validated during AST building
        // TODO: Move that here? Should we just have a string in the AST
        // and then fill the type field here?

        if let Some(val) = &mut node.value {
            self.validate(val)?;
        }

        if let Some(ty) = &node.ty {
            if let Some(val) = &node.value {
                let ret = val.returns(self.scope()?);

                if let Some(ret) = ret {
                    if ret != *ty {
                        self.errors.push(Err::TypeMismatch {
                            span: node.span(),
                            expected: ty.clone(),
                            got: ret,
                        });
                    }
                } else {
                    // We're not just gonna blindly trust it :P
                    self.errors
                        .push(Err::CannotComputeType { span: val.span() });
                }
            }
        } else if let Some(val) = &node.value {
            let ret = val.returns(self.scope()?);

            if ret.is_none() {
                // Can't infer it!
                self.errors.push(Err::CannotInferType { span: val.span() });
            }
        }

        self.scope_mut()?.add_local(node.name.clone(), node.clone());

        Ok(())
    }
}
