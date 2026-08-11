//! Basic type import resolution.

use crate::{cx::VisitCx, util::Export, visitor::DefVisitor};
use dpscript_ast::{
    prelude::{
        HasSpan,
        def::{enums::Enum, import::Import, structs::Struct},
        types::{TypeData, Typedef},
    },
    util::Remote,
};

pub struct BasicScopeResolver;

impl<'a, 'visit> DefVisitor<'a, 'visit> for BasicScopeResolver {
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
                    Export::Type(it) => {
                        let map = cx.scope.current().types.clone();

                        let name = match &it.data {
                            TypeData::Enum(it) => it.name,
                            TypeData::Struct(it) => it.name,
                            TypeData::Typedef(it) => it.name,
                        };

                        if let Some(entry) = map.read().get(&name.0) {
                            let span = entry.data.span();
                            cx.duplicate_defs(name.0, span, path.span);
                        } else {
                            map.write().insert(
                                name.0,
                                Remote {
                                    module: module.clone(),
                                    span: it.span,
                                    data: it.data.clone(),
                                },
                            );
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    fn visit_enum(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Enum<'a>) {
        let map = cx.scope.current().types.clone();

        if let Some(entry) = map.read().get(&node.name.0) {
            let span = entry.data.span();
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.write().insert(
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
        let map = cx.scope.current().types.clone();

        if let Some(entry) = map.read().get(&node.name.0) {
            let span = entry.data.span();
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.write().insert(
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
        let map = cx.scope.current().types.clone();

        if let Some(entry) = map.read().get(&node.name.0) {
            let span = entry.data.span();
            cx.duplicate_defs(node.name.0, span, node.span);
        } else {
            map.write().insert(
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
