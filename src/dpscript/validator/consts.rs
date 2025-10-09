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
        for (_, node) in self.ast.constants.clone() {
            self.validate_constant(&node)?;
        }

        Ok(())
    }

    pub fn validate_constant(&mut self, node: &ConstantNode) -> Result<()> {
        if node.ty.is_none() {
            self.warnings
                .push(Warn::ConstNoExplicitType { span: node.span() });
        }

        self.scopes.push(self.global_scope.clone());
        self.validate(&node.value)?;
        self.scopes.pop();

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

        Ok(())
    }
}
