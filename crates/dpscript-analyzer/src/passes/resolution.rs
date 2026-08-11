//! Pass 3: Type resolution - resolves user-specified types to their definition.

use dpscript_ast::prelude::types::{ArrayKind, ResolvedTypeRef, TypeRef, TypeRefData};

use crate::{
    cx::VisitCx,
    scope::ScopeLookupTrait,
    visitor::{DefVisitor, ExprVisitor, MetaVisitor},
};

pub struct TypeResolver;

impl TypeResolver {
    fn resolve_ty<'a, 'visit>(
        &self,
        cx: &mut VisitCx<'a, 'visit>,
        ty: &TypeRef<'a>,
    ) -> Option<Box<ResolvedTypeRef<'a>>> {
        match &ty.data {
            TypeRefData::Named { name } => {
                let lkp = cx.lookup();
                let ty = lkp.lookup_type(name.0);

                match ty {
                    Some(res) => Some(Box::new(ResolvedTypeRef {
                        module: res.module.clone(),
                        span: res.span,
                        data: res.data.clone(),
                        array: ArrayKind::None,
                    })),

                    None => {
                        drop(ty);
                        cx.unresolved_type(name.0, name.1);
                        None
                    }
                }
            }

            TypeRefData::SizedArray { inner, .. } => self.resolve_ty(cx, &*inner).map(|mut it| {
                it.array = ArrayKind::Sized;
                it
            }),

            TypeRefData::UnsizedArray { inner } => self.resolve_ty(cx, &*inner).map(|mut it| {
                it.array = ArrayKind::Unsized;
                it
            }),

            TypeRefData::Resolved => ty.resolved.clone(),
        }
    }
}

impl<'a, 'visit> MetaVisitor<'a, 'visit> for TypeResolver {
    fn visit_type(&mut self, cx: &mut VisitCx<'a, 'visit>, ty: &mut TypeRef<'a>) {
        if ty.resolved.is_none() {
            ty.resolved = self.resolve_ty(cx, ty);
        }
    }
}

impl<'a, 'visit> ExprVisitor<'a, 'visit> for TypeResolver {
    fn meta_visitor(&mut self) -> Option<&mut dyn MetaVisitor<'a, 'visit>> {
        Some(self)
    }
}

impl<'a, 'visit> DefVisitor<'a, 'visit> for TypeResolver {
    fn meta_visitor(&mut self) -> Option<&mut dyn MetaVisitor<'a, 'visit>> {
        Some(self)
    }

    fn expr_visitor(&mut self) -> Option<&mut dyn ExprVisitor<'a, 'visit>> {
        Some(self)
    }
}
