//! Pass 2: Top-level scope resolution pass (imports, declarations, etc.).

use crate::{
    cx::VisitCx,
    util::Export,
    visitor::{DefVisitor, ExprVisitor},
};
use dpscript_ast::{
    prelude::{
        def::{constant::Constant, func::Function, import::Import, objective::Objective},
        expr::var::Variable,
    },
    util::Remote,
};

pub struct TopScopeResolver;

impl<'a, 'visit> DefVisitor<'a, 'visit> for TopScopeResolver {
    fn visit_import(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Import<'a>) {
        for path in &node.paths {
            let mut module = String::new();
            let parts = path.parts.len() - 1;

            for i in 0..parts {
                let (part, _) = path.parts[i];

                module.push_str(part);

                if i < parts - 1 {
                    module.push_str("::");
                }
            }

            let name = path.parts.last().unwrap();

            let Some(target) = cx.analysis.modules.get(&module) else {
                cx.cannot_find_module(module, path.span);
                continue;
            };

            if let Some(export) = target.exports.get(name.0) {
                match export {
                    Export::Constant(it) => {
                        let map = &mut cx.scope.current().consts;

                        if let Some(entry) = map.get(&it.data.name.0) {
                            let span = entry.span;
                            cx.duplicate_defs(it.data.name.0, span, path.span);
                        } else {
                            map.insert(
                                it.data.name.0,
                                Remote {
                                    module: module.clone(),
                                    span: it.span,
                                    data: it.data.clone(),
                                },
                            );
                        }
                    }

                    Export::Function(it) => {
                        if let Some(dsl) = &it.data.meta.dsl
                            && let Some(arg) = it.data.args.first()
                        {
                            let map = &mut cx.scope.current().dsl_funcs;
                            let entry = map.entry(*dsl).or_default();

                            if let Some(entry) = entry.get(&arg.ty.as_id()) {
                                let span = entry.span;
                                cx.duplicate_defs(it.data.name.0, span, path.span);
                            } else {
                                entry.insert(
                                    arg.ty.as_id(),
                                    Remote {
                                        module: module.clone(),
                                        span: it.span,
                                        data: it.data.clone(),
                                    },
                                );
                            }
                        } else if let Some(ty) = &it.data.target {
                            let map = &mut cx.scope.current().inst_funcs;
                            let entry = map.entry(it.data.name.0).or_default();

                            if let Some(entry) = entry.get(&ty.as_id()) {
                                let span = entry.span;
                                cx.duplicate_defs(it.data.name.0, span, path.span);
                            } else {
                                entry.insert(
                                    ty.as_id(),
                                    Remote {
                                        module: module.clone(),
                                        span: it.span,
                                        data: it.data.clone(),
                                    },
                                );
                            }
                        } else {
                            let map = &mut cx.scope.current().funcs;

                            if let Some(entry) = map.get(&it.data.name.0) {
                                let span = entry.span;
                                cx.duplicate_defs(it.data.name.0, span, path.span);
                            } else {
                                map.insert(
                                    it.data.name.0,
                                    Remote {
                                        module: module.clone(),
                                        span: it.span,
                                        data: it.data.clone(),
                                    },
                                );
                            }
                        }
                    }

                    Export::Objective(it) => {
                        let map = &mut cx.scope.current().objectives;

                        if let Some(entry) = map.get(&it.data.name.0) {
                            let span = entry.data.name.1;
                            cx.duplicate_defs(it.data.name.0, span, path.span);
                        } else {
                            map.insert(
                                it.data.name.0,
                                Remote {
                                    module: module.clone(),
                                    span: it.span,
                                    data: it.data.clone(),
                                },
                            );
                        }
                    }

                    // Handled in the basic resolver
                    Export::Type(_) => {}
                }
            } else if let Some(export) = target.inst_func_exports.get(name.0) {
                for (ty, it) in export.clone() {
                    let map = &mut cx.scope.current().inst_funcs;
                    let entry = map.entry(it.data.name.0).or_default();

                    if let Some(entry) = entry.get(&ty) {
                        let span = entry.span;
                        cx.duplicate_defs(it.data.name.0, span, path.span);
                    } else {
                        entry.insert(
                            ty,
                            Remote {
                                module: module.clone(),
                                span: it.span,
                                data: it.data.clone(),
                            },
                        );
                    }
                }
            } else {
                cx.unresolved_import(name.0, name.1);
            }
        }
    }

    fn visit_constant(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Constant<'a>) {
        let map = &mut cx.scope.current().consts;

        if let Some(entry) = map.get(&node.name.0) {
            let span = entry.span;
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.insert(
                node.name.0,
                Remote {
                    module: cx.module.name.clone(),
                    span: node.span,
                    data: node.clone(),
                },
            );
        }
    }

    fn visit_func(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Function<'a>) {
        if let Some(dsl) = &node.info.meta.dsl
            && let Some(arg) = node.info.args.first()
        {
            let map = &mut cx.scope.current().dsl_funcs;
            let entry = map.entry(*dsl).or_default();

            if let Some(entry) = entry.get(&arg.ty.as_id()) {
                let span = entry.span;
                cx.duplicate_defs(node.info.name.0, span, node.span);
            } else {
                entry.insert(
                    arg.ty.as_id(),
                    Remote {
                        module: cx.module.name.clone(),
                        span: node.span,
                        data: node.info.clone(),
                    },
                );
            }
        } else if let Some(ty) = &node.info.target {
            let map = &mut cx.scope.current().inst_funcs;
            let entry = map.entry(node.info.name.0).or_default();

            if let Some(entry) = entry.get(&ty.as_id()) {
                let span = entry.span;
                cx.duplicate_defs(node.info.name.0, span, node.span);
            } else {
                entry.insert(
                    ty.as_id(),
                    Remote {
                        module: cx.module.name.clone(),
                        span: node.span,
                        data: node.info.clone(),
                    },
                );
            }
        } else {
            let map = &mut cx.scope.current().funcs;

            if let Some(entry) = map.get(&node.info.name.0) {
                let span = entry.span;
                cx.duplicate_defs(node.info.name.0, span, node.span);
            } else {
                map.insert(
                    node.info.name.0,
                    Remote {
                        module: cx.module.name.clone(),
                        span: node.span,
                        data: node.info.clone(),
                    },
                );
            }
        }

        cx.scope
            .push_existing(node.scope.take().unwrap_or_default());

        for arg in &node.info.args {
            cx.scope.current().vars.insert(
                arg.name.0,
                Remote {
                    data: Variable {
                        value: None,
                        ty: Some(arg.ty.clone()),
                        name: arg.name,
                        span: arg.span,
                    },

                    module: cx.module.name.clone(),
                    span: arg.span,
                },
            );
        }

        if let Some(visitor) = self.expr_visitor() {
            for expr in &mut node.body {
                visitor.visit_expr(cx, expr);
            }
        }

        node.scope = Some(cx.scope.pop());

        if let Some(dsl) = node.info.meta.dsl
            && let Some(arg) = node.info.args.first()
            && let Some(it) = cx.scope.lookup().lookup_dsl_func_mut(&arg.ty.as_id(), dsl)
            && it.module == cx.module.name
            && it.span == node.span
        {
            if it.data != node.info {
                it.data = node.info.clone();
            }
        } else if let Some(target) = &node.info.target
            && let Some(it) = cx
                .scope
                .lookup()
                .lookup_inst_func_mut(node.info.name.0, &target.as_id())
            && it.module == cx.module.name
            && it.span == node.span
        {
            if it.data != node.info {
                it.data = node.info.clone();
            }
        } else if let Some(it) = cx.scope.lookup().lookup_func_mut(node.info.name.0)
            && it.module == cx.module.name
            && it.span == node.span
        {
            if it.data != node.info {
                it.data = node.info.clone();
            }
        }
    }

    fn visit_objective(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Objective<'a>) {
        let map = &mut cx.scope.current().objectives;

        if let Some(entry) = map.get(&node.name.0) {
            let span = entry.data.name.1;
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.insert(
                node.name.0,
                Remote {
                    module: cx.module.name.clone(),
                    span: node.span,
                    data: node.clone(),
                },
            );
        }
    }

    // enum, struct, typedef: all handled in the basic scope resolver
}

impl<'a, 'visit> ExprVisitor<'a, 'visit> for TopScopeResolver {
    fn visit_var(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Variable<'a>) {
        let map = &mut cx.scope.current().vars;

        if let Some(entry) = map.get(&node.name.0) {
            let span = entry.span;
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.insert(
                node.name.0,
                Remote {
                    module: cx.module.name.clone(),
                    span: node.span,
                    data: node.clone(),
                },
            );
        }
    }

    fn visit_constant_expr(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Constant<'a>) {
        let map = &mut cx.scope.current().consts;

        if let Some(entry) = map.get(&node.name.0) {
            let span = entry.span;
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.insert(
                node.name.0,
                Remote {
                    module: cx.module.name.clone(),
                    span: node.span,
                    data: node.clone(),
                },
            );
        }
    }
}
