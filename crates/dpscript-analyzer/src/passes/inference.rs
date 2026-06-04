//! Pass 4: Type inference - infers types of variables and values. Also constructs local scopes.

use dpscript_ast::prelude::{
    SourceSpan,
    def::func::Function,
    expr::var::Variable,
    types::{ArrayKind, ResolvedTypeRef, TypeRef, TypeRefData},
    value::Value,
};

use crate::{
    cx::VisitCx,
    ops::{BASE_TYPES_MODULE, DEFAULT_FN_RET_TY},
    visitor::{DefVisitor, ExprVisitor, ValueVisitor},
};

pub struct TypeInference;

impl TypeInference {
    fn ensure_base_type<'a, 'visit>(
        &self,
        cx: &mut VisitCx<'a, 'visit>,
        span: SourceSpan,
        name: &str,
    ) -> Option<TypeRef<'a>> {
        let ty = if cx.module.name == BASE_TYPES_MODULE {
            cx.scope.current().types.get(name).cloned()
        } else {
            let Some(target) = cx.analysis.modules.get(BASE_TYPES_MODULE) else {
                cx.cannot_find_module(BASE_TYPES_MODULE, span);

                return None;
            };

            // TODO: Do this with an export?
            target.scope.types.get(name).cloned()
        };

        if ty.is_none() {
            cx.unresolved_type(name, span);
        }

        ty.map(|it| TypeRef {
            data: TypeRefData::Resolved,
            span: it.span,
            resolved: Some(Box::new(ResolvedTypeRef {
                array: ArrayKind::None,
                module: it.module,
                span: it.span,
                data: it.data,
            })),
        })
    }
}

impl<'a, 'local> DefVisitor<'a, 'local> for TypeInference {
    fn value_visitor(&mut self) -> Option<&mut dyn ValueVisitor<'a, 'local>> {
        Some(self)
    }

    fn expr_visitor(&mut self) -> Option<&mut dyn ExprVisitor<'a, 'local>> {
        Some(self)
    }

    fn visit_func(&mut self, cx: &mut VisitCx<'a, 'local>, node: &mut Function<'a>) {
        if node.info.ret.is_none() {
            node.info.ret = self.ensure_base_type(cx, node.info.span, DEFAULT_FN_RET_TY);
        }
    }
}

impl<'a, 'local> ExprVisitor<'a, 'local> for TypeInference {
    fn value_visitor(&mut self) -> Option<&mut dyn ValueVisitor<'a, 'local>> {
        Some(self)
    }

    fn visit_var(&mut self, cx: &mut VisitCx<'a, 'local>, node: &mut Variable<'a>) {
        // TODO: If node `type` missing, replace with inferred
    }
}

impl<'a, 'local> ValueVisitor<'a, 'local> for TypeInference {
    fn visit_value(&mut self, cx: &mut VisitCx<'a, 'local>, node: &mut Value<'a>) {
        // TODO: Replace with TypedValue
    }
}
