use dpscript_ast::prelude::{
    def::{
        Def, block::Block, constant::Constant, enums::Enum, export::Export, func::Function,
        import::Import, objective::Objective, structs::Struct,
    },
    types::{TypeData, Typedef},
};

use crate::{
    cx::VisitCx,
    scope::ScopeLookupMutTrait,
    visitor::{ExprVisitor, MetaVisitor, ValueVisitor},
};

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

        cx.scope
            .push_existing(node.scope.take().unwrap_or_default());

        if let Some(visitor) = self.expr_visitor() {
            for expr in &mut node.body {
                visitor.visit_expr(cx, expr);
            }
        }

        node.scope = Some(cx.scope.pop());
    }

    fn visit_constant(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Constant<'a>) {
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

    fn visit_enum(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Enum<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
        }

        for variant in &mut node.variants {
            if let Some(visitor) = self.meta_visitor() {
                visitor.visit_meta(cx, &mut variant.meta);
            }
        }

        let module = cx.module.name.clone();

        if let Some(mut it) = cx.lookup_mut().lookup_type_mut(node.name.0)
            && it.module == module
            && it.span == node.span
            && let TypeData::Enum(obj) = &mut it.data
            && *obj != *node
        {
            *obj = node.clone();
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

        cx.scope
            .push_existing(node.scope.take().unwrap_or_default());

        if let Some(visitor) = self.expr_visitor() {
            for expr in &mut node.body {
                visitor.visit_expr(cx, expr);
            }
        }

        node.scope = Some(cx.scope.pop());

        let module = cx.module.name.clone();

        if let Some(dsl) = node.info.meta.dsl
            && let Some(arg) = node.info.args.first()
            && let Some(mut it) = cx.lookup_mut().lookup_dsl_func_mut(&arg.ty.as_id(), dsl)
            && it.module == module
            && it.span == node.span
        {
            if it.data != node.info {
                it.data = node.info.clone();
            }
        } else if let Some(target) = &node.info.target
            && let Some(mut it) = cx
                .lookup_mut()
                .lookup_inst_func_mut(node.info.name.0, &target.as_id())
            && it.module == module
            && it.span == node.span
        {
            if it.data != node.info {
                it.data = node.info.clone();
            }
        } else if let Some(mut it) = cx.lookup_mut().lookup_func_mut(node.info.name.0)
            && it.module == module
            && it.span == node.span
        {
            if it.data != node.info {
                it.data = node.info.clone();
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

        let module = cx.module.name.clone();

        if let Some(mut it) = cx.lookup_mut().lookup_objective_mut(node.name.0)
            && it.module == module
            && it.span == node.span
            && it.data != *node
        {
            it.data = node.clone();
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

        let module = cx.module.name.clone();

        if let Some(mut it) = cx.lookup_mut().lookup_type_mut(node.name.0)
            && it.module == module
            && it.span == node.span
            && let TypeData::Struct(obj) = &mut it.data
            && *obj != *node
        {
            *obj = node.clone();
        }
    }

    fn visit_typedef(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Typedef<'a>) {
        if let Some(visitor) = self.meta_visitor() {
            visitor.visit_meta(cx, &mut node.meta);
        }

        let module = cx.module.name.clone();

        if let Some(mut it) = cx.lookup_mut().lookup_type_mut(node.name.0)
            && it.module == module
            && it.span == node.span
            && let TypeData::Typedef(obj) = &mut it.data
            && *obj != *node
        {
            *obj = node.clone();
        }
    }
}
