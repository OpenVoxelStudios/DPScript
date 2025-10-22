use std::collections::BTreeMap;

use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::constant::ConstantNode,
        data::NodeInfo,
        validator::{
            Result, Validator,
            err::{Err, Warn},
        },
    },
};

impl Validator {
    pub fn validate_constants(&mut self) -> Result<()> {
        let mut out = BTreeMap::new();

        for (k, mut node) in self.ast.scope.constants.clone() {
            self.validate_constant(&mut node)?;
            out.insert(k, node);
        }

        for (k, mut node) in self.scope()?.constants.clone() {
            self.validate_constant(&mut node)?;
            out.insert(k, node);
        }

        self.scope_mut()?.constants = out;

        Ok(())
    }

    pub fn validate_constant(&mut self, node: &mut ConstantNode) -> Result<()> {
        if node.ty.is_none() {
            self.warnings
                .push(Warn::ConstNoExplicitType { span: node.span() });
        }

        self.validate_ident((&node.name, node.span))?; // TODO: Better span so it's actually the name
        self.validate(&mut node.value)?;

        if !node.value.is_const(self.scope()?) {
            self.errors.push(Err::NotConstSafe {
                span: node.value.span(),
            });
        }

        let ret = node.value.returns(self.scope()?);

        if let Some(ret) = ret {
            if let Some(ty) = &node.ty {
                if ret != *ty {
                    self.errors.push(Err::TypeMismatch {
                        span: node.span(),
                        expected: ty.clone(),
                        got: ret,
                    });
                }
            }
        } else {
            // We're not just gonna blindly trust it :P
            self.errors.push(Err::CannotComputeType {
                span: node.value.span(),
            });
        }

        self.scope_mut()?
            .add_local(node.name.clone(), node.as_var());

        Ok(())
    }
}
