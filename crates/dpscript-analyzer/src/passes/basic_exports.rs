//! Basic export resolver (types only)

use crate::{cx::VisitCx, util::Export, visitor::DefVisitor};
use dpscript_ast::{
    prelude::{
        HasSpan,
        def::{enums::Enum, structs::Struct},
        meta::DefFlags,
        types::{TypeData, Typedef},
    },
    util::Remote,
};

pub struct BasicExportResolver;

impl<'a, 'visit> DefVisitor<'a, 'visit> for BasicExportResolver {
    fn visit_struct(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Struct<'a>) {
        if node.flags.contains(&DefFlags::Public) {
            if let Some(export) = cx.module.exports.get(&node.name.0) {
                cx.duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Type(Remote {
                        data: TypeData::Struct(node.clone()),
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
                cx.duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Type(Remote {
                        data: TypeData::Enum(node.clone()),
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
                cx.duplicate_export(node.name.0, export.span(), node.span);
            } else {
                cx.module.exports.insert(
                    &node.name.0,
                    Export::Type(Remote {
                        data: TypeData::Typedef(node.clone()),
                        span: node.span,
                        module: cx.module.name.clone(),
                    }),
                );
            }
        }
    }
}
