use std::{cell::RefCell, rc::Rc};

use ast::{
    loops::{LoopCondition, LoopNode},
    scope::Scope,
};

use crate::dpscript::validator::{Result, Validator};

impl<'a> Validator<'a> {
    #[allow(unused_labels)]
    pub fn validate_loop(&mut self, node: &mut LoopNode<'a>) -> Result<()> {
        'cond: {
            match &mut node.condition {
                LoopCondition::Range { var, .. } => self.validate_ident(*var)?,

                LoopCondition::Iter { var, .. } => {
                    self.validate_ident(*var)?;

                    // TODO: Type checking

                    // let Some(ty) = array.returns(self.scope()?) else {
                    //     self.errors.push(Err::CannotComputeType {
                    //         span: array.1.into(),
                    //     });

                    //     break 'cond;
                    // };

                    // if !ty.is_array() {
                    //     self.errors.push(Err::LoopNotArray {
                    //         span: array.1.into(),
                    //     });
                    // }
                }

                LoopCondition::While { span: _, condition } => {
                    self.validate(condition)?;

                    // TODO: Type checking

                    // let Some(ty) = condition.returns(self.scope()?) else {
                    //     self.errors.push(Err::CannotComputeType {
                    //         span: condition.span().into(),
                    //     });

                    //     break 'cond;
                    // };

                    // if !ty.is_truthy() {
                    //     self.errors.push(Err::CondNotBool {
                    //         span: condition.span().into(),
                    //     });
                    // }
                }
            };
        };

        debug!("Pushing scope (loop): {}", node.ident);

        self.scopes.push(Rc::new(RefCell::new(Scope::new(
            node.ident.path,
            self.scopes.clone(),
        ))));

        for node in &mut node.body {
            self.validate(node)?;
        }

        node.scope = self.scopes.pop();

        debug!("Popped scope!");

        Ok(())
    }
}
