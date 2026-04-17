use ast::var::VarNode;
use crate::dpscript::validator::{Result, Validator};

impl<'a> Validator<'a> {
    pub fn validate_variable(&mut self, node: &mut VarNode<'a>) -> Result<()> {
        self.validate_ident(node.name)?; // TODO: Improve the span so it's actually just the name

        // types should be automatically validated during AST building
        // TODO: Move that here? Should we just have a string in the AST
        // and then fill the type field here?

        if let Some(val) = &mut node.value {
            self.validate(val)?;
        }

        // TODO: Type checking

        // if let Some(ty) = &node.ty {
        //     if let Some(val) = &node.value {
        //         let ret = val.returns(self.scope()?);

        //         if let Some(ret) = ret {
        //             if ret != *ty {
        //                 self.errors.push(Err::TypeMismatch {
        //                     span: node.span().into(),
        //                     expected: ty.0,
        //                     got: ret,
        //                 });
        //             }
        //         } else {
        //             // We're not just gonna blindly trust it :P
        //             self.errors.push(Err::CannotComputeType {
        //                 span: val.span().into(),
        //             });
        //         }
        //     }
        // } else if let Some(val) = &node.value {
        //     let ret = val.returns(self.scope()?);

        //     if ret.is_none() {
        //         // Can't infer it!
        //         self.errors.push(Err::CannotInferType {
        //             span: val.span().into(),
        //         });
        //     }
        // }

        self.scope()?
            .borrow_mut()
            .add_local(node.name.0, node.clone());

        Ok(())
    }
}
