//! Pass 1: Export resolution & module completion.

use crate::{
    cx::VisitCx,
    util::{
        ConstExport, EnumExport, Export, FuncExport, ObjectiveExport, StructExport, TypedefExport,
    },
    visitor::DefVisitor,
};
use dpscript_ast::prelude::{
    HasSpan,
    def::{
        constant::Constant, enums::Enum, export::Export as ExportNode, func::Function,
        objective::Objective, structs::Struct,
    },
    meta::DefFlags,
    types::Typedef,
};

pub struct ExportResolver;

impl<'a, 'visit> DefVisitor<'a, 'visit> for ExportResolver
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
                cx.analysis.cannot_find_module(module, path.span);

                continue;
            }

            let module = cx.analysis.modules.get(&module).unwrap();

            // TODO: Don't clone this!
            for (k, v) in module.exports.clone() {
                if let Some(export) = cx.module.exports.get(k) {
                    cx.analysis.duplicate_export(k, export.span(), path.span);
                } else {
                    cx.module.exports.insert(k, v.with(path.span, v.module()));
                }
            }
        }
    }

    fn visit_constant(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Constant<'a>) {
        if node.flags.contains(&DefFlags::Public) {
            if let Some(export) = cx.module.exports.get(&node.name.0) {
                cx.analysis
                    .duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Constant(ConstExport {
                        name: node.name,
                        ty: node.ty.clone(),
                        meta: node.meta.clone(),
                        span: node.span,
                        module: cx.module.name.clone(),
                    }),
                );
            }
        }
    }

    fn visit_func(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Function<'a>) {
        if node.info.flags.contains(&DefFlags::Public) {
            if let Some(export) = cx.module.exports.get(&node.info.name.0) {
                cx.analysis
                    .duplicate_export(node.info.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.info.name.0,
                    Export::Function(FuncExport {
                        info: node.info.clone(),
                        meta: node.info.meta.clone(),
                        span: node.span,
                        module: cx.module.name.clone(),
                    }),
                );
            }
        }
    }

    fn visit_objective(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Objective<'a>) {
        if node.flags.contains(&DefFlags::Public) {
            if let Some(export) = cx.module.exports.get(&node.name.0) {
                cx.analysis
                    .duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Objective(ObjectiveExport {
                        name: node.name,
                        meta: node.meta.clone(),
                        span: node.span,
                        module: cx.module.name.clone(),
                    }),
                );
            }
        }
    }

    fn visit_struct(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Struct<'a>) {
        if node.flags.contains(&DefFlags::Public) {
            if let Some(export) = cx.module.exports.get(&node.name.0) {
                cx.analysis
                    .duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Struct(StructExport {
                        name: node.name,
                        meta: node.meta.clone(),
                        extends: node.extends.clone(),
                        fields: node.fields.clone(),
                        span: node.span,
                        module: cx.module.name.clone(),
                    }),
                );
            }
        }
    }

    fn visit_enum(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Enum<'a>) {
        if node.flags.contains(&DefFlags::Public) {
            if let Some(export) = cx.module.exports.get(&node.name.0) {
                cx.analysis
                    .duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Enum(EnumExport {
                        name: node.name,
                        variants: node.variants.clone(),
                        meta: node.meta.clone(),
                        span: node.span,
                        module: cx.module.name.clone(),
                    }),
                );
            }
        }
    }

    fn visit_typedef(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Typedef<'a>) {
        if node.flags.contains(&DefFlags::Public) {
            if let Some(export) = cx.module.exports.get(&node.name.0) {
                cx.analysis
                    .duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Typedef(TypedefExport {
                        name: node.name,
                        meta: node.meta.clone(),
                        span: node.span,
                        module: cx.module.name.clone(),
                    }),
                );
            }
        }
    }
}
