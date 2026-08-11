use dpscript_ast::{
    nodes::def::constant::Constant,
    prelude::expr::{
        Expr,
        assign::Assign,
        block::{at::At, cond::Cond, loops::ForLoop},
        call::Call,
        ret::Return,
        var::Variable,
    },
};

use crate::{
    cx::VisitCx,
    scope::ScopeLookupMutTrait,
    visitor::{MetaVisitor, ValueVisitor},
};

pub trait ExprVisitor<'a, 'visit> {
    fn value_visitor(&mut self) -> Option<&mut dyn ValueVisitor<'a, 'visit>> {
        None
    }

    fn meta_visitor(&mut self) -> Option<&mut dyn MetaVisitor<'a, 'visit>> {
        None
    }

    fn visit_expr(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Expr<'a>) {
        match node {
            Expr::Call(it) => self.visit_call(cx, it),
            Expr::At(it) => self.visit_at(cx, it),
            Expr::Cond(it) => self.visit_cond(cx, it),
            Expr::ForLoop(it) => self.visit_for_loop(cx, it),
            Expr::Return(it) => self.visit_return(cx, it),
            Expr::Variable(it) => self.visit_var(cx, it),
            Expr::Constant(it) => self.visit_constant_expr(cx, it),
            Expr::Assign(it) => self.visit_assign(cx, it),
        }
    }

    fn visit_at(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut At<'a>) {
        if let Some(visitor) = self.value_visitor() {
            visitor.visit_value(cx, &mut node.arg);
        }

        cx.scope
            .push_existing(node.scope.take().unwrap_or_default());

        for expr in &mut node.body {
            self.visit_expr(cx, expr);
        }

        node.scope = Some(cx.scope.pop());
    }

    fn visit_cond(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Cond<'a>) {
        if let Some(visitor) = self.value_visitor() {
            for cond in &mut node.conditions {
                visitor.visit_value(cx, &mut cond.condition);
            }
        }

        for cond in &mut node.conditions {
            cx.scope
                .push_existing(cond.scope.take().unwrap_or_default());

            for expr in &mut cond.body {
                self.visit_expr(cx, expr);
            }

            cond.scope = Some(cx.scope.pop());
        }

        cx.scope
            .push_existing(node.else_scope.take().unwrap_or_default());

        for expr in &mut node.else_block {
            self.visit_expr(cx, expr);
        }

        node.else_scope = Some(cx.scope.pop());
    }

    fn visit_constant_expr(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Constant<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
            visitor.visit_type(cx, &mut node.ty);
        }

        if let Some(visitor) = self.value_visitor() {
            visitor.visit_value(cx, &mut node.value);
        }

        let module = cx.module.name.clone();

        if let Some((mut it, _)) = cx.lookup_mut().lookup_const_mut(node.name.0)
            && it.module == module
            && it.span == node.span
            && it.data != *node
        {
            it.data = node.clone();
        }
    }

    fn visit_for_loop(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut ForLoop<'a>) {
        if let Some(visitor) = self.value_visitor() {
            visitor.visit_value(cx, &mut node.array);
        }

        cx.scope
            .push_existing(node.scope.take().unwrap_or_default());

        for expr in &mut node.body {
            self.visit_expr(cx, expr);
        }

        node.scope = Some(cx.scope.pop());
    }

    fn visit_assign(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Assign<'a>) {
        if let Some(visitor) = self.value_visitor() {
            visitor.visit_value(cx, &mut node.lhs);
            visitor.visit_value(cx, &mut node.rhs);
        }
    }

    fn visit_call(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Call<'a>) {
        if let Some(visitor) = self.value_visitor() {
            if let Some(target) = &mut node.target {
                visitor.visit_value(cx, target);
            }

            for arg in &mut node.args {
                visitor.visit_value(cx, arg);
            }
        }
    }

    fn visit_return(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Return<'a>) {
        if let Some(visitor) = self.value_visitor() {
            if let Some(value) = &mut node.value {
                visitor.visit_value(cx, value);
            }
        }
    }

    fn visit_var(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Variable<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            if let Some(ty) = &mut node.ty {
                visitor.visit_type(cx, ty);
            }
        }

        if let Some(visitor) = self.value_visitor() {
            if let Some(value) = &mut node.value {
                visitor.visit_value(cx, value);
            }
        }

        let module = cx.module.name.clone();

        if let Some((mut it, _)) = cx.lookup_mut().lookup_var_mut(node.name.0)
            && it.module == module
            && it.span == node.span
            && it.data != *node
        {
            it.data = node.clone();
        }
    }
}
