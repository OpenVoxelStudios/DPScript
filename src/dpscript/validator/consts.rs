use crate::dpscript::validator::{Result, Validator, err::Warn};
use ast::{constant::ConstantNode, data::HasSpan};
use std::{collections::BTreeMap, rc::Rc};

impl<'a> Validator<'a> {
    pub fn validate_constants(&mut self) -> Result<()> {
        let mut out = BTreeMap::new();
        let cs = self.ast.borrow().scope.borrow().constants.clone();

        for (k, node) in cs {
            let copy = ConstantNode::clone(&node);
            let copy = self.validate_constant(copy)?;
            out.insert(k, copy);
        }

        let cs = self.scope()?.borrow().constants.clone();

        for (k, node) in cs {
            let copy = ConstantNode::clone(&node);
            let copy = self.validate_constant(copy)?;
            out.insert(k, copy);
        }

        self.scope()?.borrow_mut().constants = out;

        Ok(())
    }

    pub fn validate_constant(
        &mut self,
        mut node: ConstantNode<'a>,
    ) -> Result<Rc<ConstantNode<'a>>> {
        if node.ty.is_none() {
            self.warnings.push(Warn::ConstNoExplicitType {
                span: node.span().into(),
            });
        }

        self.validate_ident(node.name)?; // TODO: Better span so it's actually the name
        self.validate(&mut node.value)?;

        // TODO: Const safety & type checking

        // if !node.value.is_const(self.scope()?) {
        //     self.errors.push(Err::NotConstSafe {
        //         span: node.value.span().into(),
        //     });
        // }

        // let ret = node.value.returns(self.scope()?);

        // if let Some(ret) = ret {
        //     if let Some(ty) = &node.ty {
        //         if ret != *ty {
        //             self.errors.push(Err::TypeMismatch {
        //                 span: node.span().into(),
        //                 expected: ty.0,
        //                 got: ret,
        //             });
        //         }
        //     }
        // } else {
        //     // We're not just gonna blindly trust it :P
        //     self.errors.push(Err::CannotComputeType {
        //         span: node.value.span().into(),
        //     });
        // }

        let ref_ = Rc::new(node.clone());

        self.scope()?
            .borrow_mut()
            .constants
            .insert(node.name.0, Rc::clone(&ref_));

        Ok(ref_)
    }
}
