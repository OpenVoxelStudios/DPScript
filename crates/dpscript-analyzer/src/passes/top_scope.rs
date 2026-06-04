//! Pass 2: Top-level scope resolution pass (imports, declarations, etc.).

use crate::{cx::VisitCx, util::Export, visitor::DefVisitor};
use dpscript_ast::{
    prelude::{
        HasSpan,
        def::{
            constant::Constant, enums::Enum, func::Function, import::Import, objective::Objective,
            structs::Struct,
        },
        types::{TypeData, Typedef},
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

                        if let Some(entry) = map.get(&it.name.0) {
                            let span = entry.span;
                            cx.duplicate_defs(it.name.0, span, path.span);
                        } else {
                            map.insert(it.name.0, it.ty.clone());
                        }
                    }

                    Export::Function(it) => {
                        if let Some(ty) = &it.info.target {
                            let map = &mut cx.scope.current().inst_funcs;
                            let entry = map.entry(it.info.name.0).or_default();

                            if let Some(entry) = entry.get(&ty.to_string()) {
                                let span = entry.span;
                                cx.duplicate_defs(it.info.name.0, span, path.span);
                            } else {
                                entry.insert(ty.to_string(), it.info.clone());
                            }
                        } else {
                            let map = &mut cx.scope.current().funcs;

                            if let Some(entry) = map.get(&it.info.name.0) {
                                let span = entry.span;
                                cx.duplicate_defs(it.info.name.0, span, path.span);
                            } else {
                                map.insert(it.info.name.0, it.info.clone());
                            }
                        }
                    }

                    Export::Objective(it) => {
                        let map = &mut cx.scope.current().objectives;

                        if let Some(entry) = map.get(&it.name.0) {
                            let span = entry.1;
                            cx.duplicate_defs(it.name.0, span, path.span);
                        } else {
                            map.insert(it.name.0, it.name);
                        }
                    }

                    Export::Type(it) => {
                        let map = &mut cx.scope.current().types;

                        if let Some(entry) = map.get(&it.name.0) {
                            let span = entry.data.span();
                            cx.duplicate_defs(it.name.0, span, path.span);
                        } else {
                            map.insert(
                                it.name.0,
                                Remote {
                                    module: module.clone(),
                                    span: it.span,
                                    data: it.data.clone(),
                                },
                            );
                        }
                    }
                }
            } else if let Some(export) = target.inst_func_exports.get(name.0) {
                for (ty, it) in export.clone() {
                    let map = &mut cx.scope.current().inst_funcs;
                    let entry = map.entry(it.info.name.0).or_default();

                    if let Some(entry) = entry.get(&ty.to_string()) {
                        let span = entry.span;
                        cx.duplicate_defs(it.info.name.0, span, path.span);
                    } else {
                        entry.insert(ty.to_string(), it.info.clone());
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
            map.insert(node.name.0, node.ty.clone());
        }
    }

    fn visit_func(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Function<'a>) {
        if let Some(ty) = &node.info.target {
            let map = &mut cx.scope.current().inst_funcs;
            let entry = map.entry(node.info.name.0).or_default();

            if let Some(entry) = entry.get(&ty.to_string()) {
                let span = entry.span;
                cx.duplicate_defs(node.info.name.0, span, node.span);
            } else {
                entry.insert(ty.to_string(), node.info.clone());
            }
        } else {
            let map = &mut cx.scope.current().funcs;

            if let Some(entry) = map.get(&node.info.name.0) {
                let span = entry.span;
                cx.duplicate_defs(node.info.name.0, span, node.span);
            } else {
                map.insert(node.info.name.0, node.info.clone());
            }
        }
    }

    fn visit_objective(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Objective<'a>) {
        let map = &mut cx.scope.current().objectives;

        if let Some(entry) = map.get(&node.name.0) {
            let span = entry.1;
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.insert(node.name.0, node.name);
        }
    }

    fn visit_enum(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Enum<'a>) {
        let map = &mut cx.scope.current().types;

        if let Some(entry) = map.get(&node.name.0) {
            let span = entry.data.span();
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.insert(
                node.name.0,
                Remote {
                    module: cx.module.name.clone(),
                    span: node.span(),
                    data: TypeData::Enum(node.clone()),
                },
            );
        }
    }

    fn visit_struct(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Struct<'a>) {
        let map = &mut cx.scope.current().types;

        if let Some(entry) = map.get(&node.name.0) {
            let span = entry.data.span();
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.insert(
                node.name.0,
                Remote {
                    module: cx.module.name.clone(),
                    span: node.span(),
                    data: TypeData::Struct(node.clone()),
                },
            );
        }
    }

    fn visit_typedef(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Typedef<'a>) {
        let map = &mut cx.scope.current().types;

        if let Some(entry) = map.get(&node.name.0) {
            let span = entry.data.span();
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.insert(
                node.name.0,
                Remote {
                    module: cx.module.name.clone(),
                    span: node.span(),
                    data: TypeData::Typedef(node.clone()),
                },
            );
        }
    }
}
