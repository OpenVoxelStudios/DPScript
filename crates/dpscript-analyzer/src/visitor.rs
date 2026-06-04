use dpscript_ast::prelude::{
    def::{
        Def, block::Block, constant::Constant, enums::Enum, export::Export, func::Function,
        import::Import, objective::Objective, structs::Struct,
    },
    expr::{
        Expr,
        assign::Assign,
        block::{at::At, cond::Cond, loops::ForLoop},
        call::Call,
        ret::Return,
        var::Variable,
    },
    meta::DefMeta,
    types::{TypeRef, Typedef},
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

pub trait MetaVisitor<'a, 'visit> {
    fn visit_meta(&mut self, _cx: &mut VisitCx<'a, 'visit>, _meta: &mut DefMeta<'a>) {}

    fn visit_type(&mut self, _cx: &mut VisitCx<'a, 'visit>, _ty: &mut TypeRef<'a>) {}
}

pub trait DefVisitor<'a, 'visit> {
    fn expr_visitor(&mut self) -> Option<&mut dyn ExprVisitor<'a, 'visit>> {
        None
    }

    fn value_visitor(&mut self) -> Option<&mut dyn ValueVisitor<'a, 'visit>> {
        None
    }

    fn meta_visitor(&mut self) -> Option<&mut dyn MetaVisitor<'a, 'visit>> {
        None
    }

    fn visit_def(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Def<'a>) {
        match node {
            Def::Constant(it) => self.visit_constant(cx, it),
            Def::Enum(it) => self.visit_enum(cx, it),
            Def::Function(it) => self.visit_func(cx, it),
            Def::Import(it) => self.visit_import(cx, it),
            Def::Objective(it) => self.visit_objective(cx, it),
            Def::Struct(it) => self.visit_struct(cx, it),
            Def::Block(it) => self.visit_block(cx, it),
            Def::Typedef(it) => self.visit_typedef(cx, it),
            Def::Export(it) => self.visit_export(cx, it),
        }
    }

    fn visit_block(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Block<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
        }

        if let Some(visitor) = self.expr_visitor() {
            for expr in &mut node.body {
                visitor.visit_expr(cx, expr);
            }
        }
    }

    fn visit_constant(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Constant<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
            visitor.visit_type(cx, &mut node.ty);
        }

        if let Some(visitor) = self.value_visitor() {
            visitor.visit_value(cx, &mut node.value);
        }
    }

    fn visit_enum(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Enum<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
        }

        for variant in &mut node.variants {
            if let Some(visitor) = self.meta_visitor() {
                visitor.visit_meta(cx, &mut variant.meta);
            }
        }
    }

    fn visit_export(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Export<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
        }
    }

    fn visit_func(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Function<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.info.meta);

            for arg in &mut node.info.args {
                visitor.visit_type(cx, &mut arg.ty);
            }

            if let Some(ret) = &mut node.info.ret {
                visitor.visit_type(cx, ret);
            }
        }

        if let Some(visitor) = self.expr_visitor() {
            for expr in &mut node.body {
                visitor.visit_expr(cx, expr);
            }
        }
    }

    fn visit_import(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Import<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
        }
    }

    fn visit_objective(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Objective<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
        }
    }

    fn visit_struct(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Struct<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);

            for field in &mut node.fields {
                visitor.visit_meta(cx, &mut field.meta);
                visitor.visit_type(cx, &mut field.ty);
            }
        }
    }

    fn visit_typedef(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Typedef<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
        }
    }
}

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

        for expr in &mut node.body {
            self.visit_expr(cx, expr);
        }
    }

    fn visit_cond(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Cond<'a>) {
        if let Some(visitor) = self.value_visitor() {
            for cond in &mut node.conditions {
                visitor.visit_value(cx, &mut cond.condition);
            }
        }

        for cond in &mut node.conditions {
            for expr in &mut cond.body {
                self.visit_expr(cx, expr);
            }
        }

        for expr in &mut node.else_block {
            self.visit_expr(cx, expr);
        }
    }

    fn visit_constant_expr(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Constant<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
            visitor.visit_type(cx, &mut node.ty);
        }

        if let Some(visitor) = self.value_visitor() {
            visitor.visit_value(cx, &mut node.value);
        }
    }

    fn visit_for_loop(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut ForLoop<'a>) {
        if let Some(visitor) = self.value_visitor() {
            visitor.visit_value(cx, &mut node.array);
        }

        for expr in &mut node.body {
            self.visit_expr(cx, expr);
        }
    }

    fn visit_assign(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Assign<'a>) {
        if let Some(visitor) = self.value_visitor() {
            visitor.visit_value(cx, &mut node.lhs);
            visitor.visit_value(cx, &mut node.rhs);
        }
    }

    fn visit_call(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Call<'a>) {
        if let Some(visitor) = self.value_visitor() {
            visitor.visit_value(cx, &mut node.target);

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
    }
}

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
        self.visit_value(cx, &mut node.target);

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
