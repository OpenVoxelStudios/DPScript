use dpscript_ast::prelude::{
    expr::call::Call,
    value::{
        Value,
        arr::ArrayLiteral,
        binop::BinOp,
        literal::{DslLiteral, Literal},
        nbt::NbtLiteral,
        refs::{ValueRef, VarRef},
        unary::Unary,
    },
};

use crate::cx::VisitCx;

pub trait ValueVisitor<'a, 'visit> {
    fn visit_value(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Value<'a>) {
        match node {
            Value::BinOp(it) => self.visit_binop(cx, it),
            Value::Literal(it) => self.visit_literal(cx, it),
            Value::DslLiteral(it) => self.visit_dsl_literal(cx, it),
            Value::Call(it) => self.visit_call_value(cx, it),
            Value::ValueRef(it) => self.visit_value_ref(cx, it),
            Value::VarRef(it) => self.visit_var_ref(cx, it),
            Value::Unary(it) => self.visit_unary(cx, it),
            Value::NbtLiteral(it) => self.visit_nbt_literal(cx, it),
            Value::ArrayLiteral(it) => self.visit_arr_literal(cx, it),
            Value::TypedValue(it) => self.visit_value(cx, &mut it.value),
        }
    }

    fn visit_arr_literal(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut ArrayLiteral<'a>) {
        for value in &mut node.values {
            self.visit_value(cx, value);
        }
    }

    fn visit_binop(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut BinOp<'a>) {
        self.visit_value(cx, &mut node.lhs);
        self.visit_value(cx, &mut node.rhs);
    }

    fn visit_call_value(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Call<'a>) {
        if let Some(target) = &mut node.target {
            self.visit_value(cx, target);
        }

        for value in &mut node.args {
            self.visit_value(cx, value);
        }
    }

    fn visit_dsl_literal(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut DslLiteral<'a>) {
        self.visit_value(cx, &mut node.value);
    }

    fn visit_literal(&mut self, _cx: &mut VisitCx<'a, 'visit>, _node: &mut Literal<'a>) {}

    fn visit_nbt_literal(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut NbtLiteral<'a>) {
        for (_, value) in &mut node.values {
            self.visit_value(cx, value);
        }
    }

    fn visit_value_ref(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut ValueRef<'a>) {
        self.visit_value(cx, &mut node.root);
    }

    fn visit_var_ref(&mut self, _cx: &mut VisitCx<'a, 'visit>, _node: &mut VarRef<'a>) {}

    fn visit_unary(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Unary<'a>) {
        self.visit_value(cx, &mut node.value);
    }
}
