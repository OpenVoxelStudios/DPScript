use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            ast::Scope,
            loops::{LoopCondition, LoopNode},
        },
        data::NodeInfo,
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_loop(&mut self, node: &mut LoopNode) -> Result<()> {
        'cond: {
            match &mut node.condition {
                LoopCondition::Range {
                    span: _,
                    var,
                    min: _,
                    max: _,
                } => self.validate_ident((&var.ident, var.span))?,

                LoopCondition::Iter {
                    span: _,
                    var,
                    array,
                } => {
                    self.validate_ident((&var.ident, var.span))?;

                    let Some(ty) = array.returns(self.scope()?) else {
                        self.errors
                            .push(Err::CannotComputeType { span: array.span() });

                        break 'cond;
                    };

                    if !ty.is_array() {
                        self.errors.push(Err::LoopNotArray { span: array.span() });
                    }
                }

                LoopCondition::While { span: _, condition } => {
                    self.validate(condition)?;

                    let Some(ty) = condition.returns(self.scope()?) else {
                        self.errors.push(Err::CannotComputeType {
                            span: condition.span(),
                        });

                        break 'cond;
                    };

                    if !ty.is_truthy() {
                        self.errors.push(Err::CondNotBool {
                            span: condition.span(),
                        });
                    }
                }
            };
        };

        debug!("Pushing scope (loop): {}", node.ident);

        self.scopes.push(Scope::new(
            format!("{}", node.ident).into(),
            self.scopes.clone(),
        ));

        for node in &mut node.body {
            self.validate(node)?;
        }

        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        Ok(())
    }
}
