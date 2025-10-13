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

        self.ast.scope.constants = out;

        Ok(())
    }

    pub fn validate_constant(&mut self, node: &mut ConstantNode) -> Result<()> {
        if node.ty.is_none() {
            self.warnings
                .push(Warn::ConstNoExplicitType { span: node.span() });
        }

        self.validate_ident(&(node.name.clone(), node.span))?; // TODO: Better span so it's actually the name
        self.validate(&mut node.value)?;

        let ret = node.value.returns(&self.global_scope);

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
            self.errors.push(Err::CannotComputeType {
                span: node.value.span(),
            });
        }

        self.scope_mut()?
            .locals
            .insert(node.name.clone(), node.as_var());

        Ok(())
    }
}
