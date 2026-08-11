//! Performs AST lowering to desugar code.

use dpscript_ast::{nodes::expr::call::Call, prelude::value::Value};

use crate::{
    cx::VisitCx,
    ops::{op_to_func, unary_op_to_func},
    visitor::{DefVisitor, ExprVisitor, ValueVisitor},
};

pub struct AstLowering;

impl<'a, 'visit> DefVisitor<'a, 'visit> for AstLowering {
    fn expr_visitor(&mut self) -> Option<&mut dyn ExprVisitor<'a, 'visit>> {
        Some(self)
    }

    fn value_visitor(&mut self) -> Option<&mut dyn ValueVisitor<'a, 'visit>> {
        Some(self)
    }
}

impl<'a, 'visit> ExprVisitor<'a, 'visit> for AstLowering {
    fn value_visitor(&mut self) -> Option<&mut dyn ValueVisitor<'a, 'visit>> {
        Some(self)
    }
}

impl<'a, 'visit> ValueVisitor<'a, 'visit> for AstLowering {
    fn visit_value(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Value<'a>) {
        match node {
            Value::BinOp(it) => {
                self.visit_binop(cx, it);

                let func = op_to_func(it.op);
                let it = core::mem::take(it);

                let new = Call {
                    args: vec![*it.rhs],
                    func: (func, it.span),
                    span: it.span,
                    target: Some(it.lhs),
                    resolved: None,
                };

                *node = Value::Call(new);
            }

            Value::Unary(it) => {
                self.visit_unary(cx, it);

                let func = unary_op_to_func(it.op);
                let it = core::mem::take(it);

                let new = Call {
                    args: vec![],
                    func: (func, it.span),
                    span: it.span,
                    target: Some(it.value),
                    resolved: None,
                };

                *node = Value::Call(new);
            }

            Value::DslLiteral(it) => self.visit_dsl_literal(cx, it),

            Value::Literal(it) => self.visit_literal(cx, it),
            Value::Call(it) => self.visit_call_value(cx, it),
            Value::ValueRef(it) => self.visit_value_ref(cx, it),
            Value::VarRef(it) => self.visit_var_ref(cx, it),
            Value::NbtLiteral(it) => self.visit_nbt_literal(cx, it),
            Value::ArrayLiteral(it) => self.visit_arr_literal(cx, it),
            Value::TypedValue(it) => self.visit_value(cx, &mut it.value),
        }
    }
}
