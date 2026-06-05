//! Pass 1: Export resolution & module completion.

use crate::{cx::VisitCx, util::Export, visitor::DefVisitor};
use dpscript_ast::{
    prelude::{
        HasSpan,
        def::{
            constant::Constant, export::Export as ExportNode, func::Function, objective::Objective,
        },
        meta::DefFlags,
    },
    util::Remote,
};

pub struct ExportResolver;

impl<'a, 'visit> DefVisitor<'a, 'visit> for ExportResolver {
    fn visit_constant(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Constant<'a>) {
        if node.flags.contains(&DefFlags::Public) {
            if let Some(export) = cx.module.exports.get(&node.name.0) {
                cx.duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Constant(Remote {
                        data: node.clone(),
                        span: node.span,
                        module: cx.module.name.clone(),
                    }),
                );
            }
        }
    }

    fn visit_func(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Function<'a>) {
        if node.info.flags.contains(&DefFlags::Public) {
            if let Some(ty) = &node.info.target {
                let map = cx
                    .module
                    .inst_func_exports
                    .entry(&node.info.name.0)
                    .or_default();

                if let Some(export) = map.get(&ty.as_id()) {
                    let span = export.span();
                    cx.duplicate_export(node.info.name.0, span, node.span);
                } else {
                    map.insert(
                        ty.as_id(),
                        Remote {
                            data: node.info.clone(),
                            span: node.span,
                            module: cx.module.name.clone(),
                        },
                    );
                }
            } else {
                if let Some(export) = cx.module.exports.get(&node.info.name.0) {
                    cx.duplicate_export(node.info.name.0, export.span(), node.span);
                } else {
                    cx.module.exports.insert(
                        &node.info.name.0,
                        Export::Function(Remote {
                            data: node.info.clone(),
                            span: node.span,
                            module: cx.module.name.clone(),
                        }),
                    );
                }
            }
        }
    }

    fn visit_objective(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Objective<'a>) {
        if node.flags.contains(&DefFlags::Public) {
            if let Some(export) = cx.module.exports.get(&node.name.0) {
                cx.duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Objective(Remote {
                        data: node.clone(),
                        span: node.span,
                        module: cx.module.name.clone(),
                    }),
                );
            }
        }
    }

    // struct, enum, typedef: all handled in the basic export resolver
}

pub struct ExportStmtResolver;

impl<'a, 'visit> DefVisitor<'a, 'visit> for ExportStmtResolver
where
    'a: 'visit,
{
    fn visit_export(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut ExportNode<'a>) {
        for path in &node.paths {
            let mut module = String::new();
            let parts = path.parts.len();

            for i in 0..parts {
                let (part, _) = path.parts[i];

                module.push_str(part);

                if i < parts - 1 {
                    module.push_str("::");
                }
            }

            if !cx.analysis.visit_module(&module, self) {
                cx.cannot_find_module(module, path.span);

                continue;
            }

            // TODO: Don't clone this!
            for (k, v) in cx.analysis.modules.get(&module).unwrap().exports.clone() {
                if let Some(export) = cx.module.exports.get(k) {
                    let span = export.span();
                    cx.duplicate_export(k, span, path.span);
                } else {
                    cx.module.exports.insert(k, v.with(path.span, v.module()));
                }
            }

            for (k, v) in cx
                .analysis
                .modules
                .get(&module)
                .unwrap()
                .inst_func_exports
                .clone()
            {
                for (t, v) in v {
                    if let Some(export) = cx.module.exports.get(k) {
                        cx.duplicate_export(k, export.span(), path.span);
                    } else {
                        cx.module.inst_func_exports.entry(k).or_default().insert(
                            t,
                            Remote {
                                span: path.span,
                                ..v
                            },
                        );
                    }
                }
            }
        }
    }
}
