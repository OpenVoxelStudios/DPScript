//! Pass 5: Type inference - infers types of variables and values. Also constructs local scopes.

use dpscript_ast::{
    prelude::{
        HasSpan, SourceSpan,
        def::{
            constant::Constant,
            func::{Function, FunctionInfo},
            objective::Objective,
        },
        expr::{call::Call, var::Variable},
        meta::Repr,
        types::{ArrayKind, ResolvedTypeRef, TypeData, TypeRef, TypeRefData, TypeRefId},
        value::{
            TypedValue, Value,
            binop::BinOp,
            literal::{DslLiteral, LiteralValue},
            unary::Unary,
        },
    },
    util::{Name, Remote},
};

use crate::{
    cx::VisitCx,
    ops::{
        ANY_TYPE_NAME, BASE_TYPES_MODULE, BOOL_TYPE_NAME, BYTE_TYPE_NAME, DOUBLE_TYPE_NAME,
        FLOAT_TYPE_NAME, INT_TYPE_NAME, LONG_TYPE_NAME, NBT_TYPE_NAME, OBJECTIVE_TYPE_NAME,
        STR_TYPE_NAME, VOID_TYPE_NAME, op_to_func, unary_op_to_func,
    },
    scope::{ScopeLookupMutTrait, ScopeLookupTrait},
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
            cx.scope.current().types.read().get(name).cloned()
        } else {
            let Some(target) = cx.analysis.modules.get(BASE_TYPES_MODULE) else {
                cx.cannot_find_module(BASE_TYPES_MODULE, span);

                return None;
            };

            // TODO: Do this with an export?
            target.scope.types.read().get(name).cloned()
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

impl<'a, 'visit> DefVisitor<'a, 'visit> for TypeInference {
    fn value_visitor(&mut self) -> Option<&mut dyn ValueVisitor<'a, 'visit>> {
        Some(self)
    }

    fn expr_visitor(&mut self) -> Option<&mut dyn ExprVisitor<'a, 'visit>> {
        Some(self)
    }

    fn visit_func(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Function<'a>) {
        if node.info.ret.is_none() {
            node.info.ret = self.ensure_base_type(cx, node.info.span, VOID_TYPE_NAME);
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
}

impl<'a, 'visit> ExprVisitor<'a, 'visit> for TypeInference {
    fn value_visitor(&mut self) -> Option<&mut dyn ValueVisitor<'a, 'visit>> {
        Some(self)
    }

    fn visit_var(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Variable<'a>) {
        if node.ty.is_none()
            && let Some(value) = &mut node.value
        {
            node.ty = self.resolve_ty(cx, value);
        }
    }
}

impl TypeInference {
    fn can_assign_from<'a, 'visit>(
        &mut self,
        cx: &mut VisitCx<'a, 'visit>,
        target: &TypeRefId,
        value: &TypeRefId,
    ) -> bool {
        let lookup = cx.lookup();

        target == value
            || match (target, value) {
                (TypeRefId::Named { name }, TypeRefId::Named { name: name2 }) => {
                    if name.0 == format!("{}::{}", BASE_TYPES_MODULE, ANY_TYPE_NAME)
                        || name2.0 == format!("{}::{}", BASE_TYPES_MODULE, ANY_TYPE_NAME)
                    {
                        true
                    } else if name.0 == format!("{}::{}", BASE_TYPES_MODULE, NBT_TYPE_NAME) {
                        let ty = lookup.lookup_type(&name2.0);

                        let Some(Remote {
                            data: TypeData::Struct(s),
                            ..
                        }) = ty.as_deref()
                        else {
                            return false;
                        };

                        s.meta.repr == Repr::Default || s.meta.repr == Repr::Object
                    } else if name2.0 == format!("{}::{}", BASE_TYPES_MODULE, NBT_TYPE_NAME) {
                        let ty = lookup.lookup_type(&name.0);

                        let Some(Remote {
                            data: TypeData::Struct(s),
                            ..
                        }) = ty.as_deref()
                        else {
                            return false;
                        };

                        s.meta.repr == Repr::Default || s.meta.repr == Repr::Object
                    } else {
                        false
                    }
                }

                (TypeRefId::UnsizedArray { .. }, TypeRefId::SizedArray { .. }) => true,

                _ => false,
            }
    }

    fn find_binop_func<'a, 'visit>(
        &mut self,
        cx: &mut VisitCx<'a, 'visit>,
        node: &mut BinOp<'a>,
    ) -> Option<Remote<FunctionInfo<'a>>> {
        let lhs_ty = self.resolve_ty(cx, &mut node.lhs)?;
        let rhs_ty = self.resolve_ty(cx, &mut node.rhs)?;

        let mut func = cx
            .lookup()
            .lookup_inst_func(op_to_func(node.op), &lhs_ty.as_id())
            .as_deref()
            .cloned();

        let nbt = self
            .ensure_base_type(cx, node.span(), NBT_TYPE_NAME)?
            .as_id();

        if func.is_none() && self.can_assign_from(cx, &nbt, &lhs_ty.as_id()) {
            func = cx
                .lookup()
                .lookup_inst_func(op_to_func(node.op), &nbt)
                .as_deref()
                .cloned();
        }

        let func = func?;

        let args_ty = func
            .data
            .args
            .iter()
            .map(|it| it.ty.clone())
            .collect::<Vec<_>>();

        if args_ty.len() == 2
            && args_ty
                .first()
                .is_some_and(|it| self.can_assign_from(cx, &it.as_id(), &lhs_ty.as_id()))
            && args_ty
                .last()
                .is_some_and(|it| self.can_assign_from(cx, &it.as_id(), &rhs_ty.as_id()))
        {
            Some(func)
        } else {
            None
        }
    }

    fn find_unary_func<'a, 'visit>(
        &mut self,
        cx: &mut VisitCx<'a, 'visit>,
        node: &mut Unary<'a>,
    ) -> Option<Remote<FunctionInfo<'a>>> {
        let val_ty = self.resolve_ty(cx, &mut node.value)?;

        let func = cx
            .lookup()
            .lookup_inst_func(unary_op_to_func(node.op), &val_ty.as_id())
            .as_deref()
            .cloned()?;

        let args_ty = func
            .data
            .args
            .iter()
            .map(|it| it.ty.clone())
            .collect::<Vec<_>>();

        if args_ty.len() == 1
            && args_ty
                .first()
                .is_some_and(|it| self.can_assign_from(cx, &it.as_id(), &val_ty.as_id()))
        {
            Some(func)
        } else {
            None
        }
    }

    fn find_dsl_func<'a, 'visit>(
        &mut self,
        cx: &mut VisitCx<'a, 'visit>,
        node: &mut DslLiteral<'a>,
    ) -> Option<Remote<FunctionInfo<'a>>> {
        let val_ty = self.resolve_ty(cx, &mut node.value)?;

        let func = cx
            .lookup()
            .lookup_dsl_func(&val_ty.as_id(), node.dsl_marker)
            .as_deref()
            .cloned()?;

        let args_ty = func
            .data
            .args
            .iter()
            .map(|it| it.ty.clone())
            .collect::<Vec<_>>();

        if args_ty.len() == 1
            && args_ty
                .first()
                .is_some_and(|it| self.can_assign_from(cx, &it.as_id(), &val_ty.as_id()))
        {
            Some(func)
        } else {
            None
        }
    }

    fn resolve_call<'a, 'visit>(
        &mut self,
        cx: &mut VisitCx<'a, 'visit>,
        node: &mut Call<'a>,
    ) -> Option<Remote<FunctionInfo<'a>>> {
        cx.lookup().lookup_func(node.func.0).as_deref().cloned()
    }

    fn find_nested_struct_field_type<'a, 'visit>(
        &mut self,
        _cx: &mut VisitCx<'a, 'visit>,
        root_ty: &TypeRef<'a>,
        field: Name<'a>,
    ) -> Option<TypeRef<'a>> {
        if let Some(ty) = &root_ty.resolved {
            if let TypeData::Struct(s) = &ty.data {
                s.fields
                    .iter()
                    .find(|it| it.name.0 == field.0)
                    .map(|it| it.ty.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn find_var<'a, 'visit>(
        &mut self,
        cx: &mut VisitCx<'a, 'visit>,
        name: Name<'a>,
    ) -> Option<Remote<Variable<'a>>> {
        cx.lookup()
            .lookup_var(name.0)
            .map(|it| it.0)
            .as_deref()
            .cloned()
    }

    fn find_const<'a, 'visit>(
        &mut self,
        cx: &mut VisitCx<'a, 'visit>,
        name: Name<'a>,
    ) -> Option<Remote<Constant<'a>>> {
        cx.lookup()
            .lookup_const(name.0)
            .map(|it| it.0)
            .as_deref()
            .cloned()
    }

    fn find_objective<'a, 'visit>(
        &mut self,
        cx: &mut VisitCx<'a, 'visit>,
        name: Name<'a>,
    ) -> Option<Remote<Objective<'a>>> {
        cx.lookup().lookup_objective(name.0).as_deref().cloned()
    }

    fn resolve_ty<'a, 'visit>(
        &mut self,
        cx: &mut VisitCx<'a, 'visit>,
        node: &mut Value<'a>,
    ) -> Option<TypeRef<'a>> {
        let span = node.span();

        let ty = match node {
            Value::Literal(it) => match it.value {
                LiteralValue::String(_) => self.ensure_base_type(cx, span, STR_TYPE_NAME),
                LiteralValue::Bool(_) => self.ensure_base_type(cx, span, BOOL_TYPE_NAME),
                LiteralValue::Byte(_) => self.ensure_base_type(cx, span, BYTE_TYPE_NAME),
                LiteralValue::Int(_) => self.ensure_base_type(cx, span, INT_TYPE_NAME),
                LiteralValue::Long(_) => self.ensure_base_type(cx, span, LONG_TYPE_NAME),
                LiteralValue::Float(_) => self.ensure_base_type(cx, span, FLOAT_TYPE_NAME),
                LiteralValue::Double(_) => self.ensure_base_type(cx, span, DOUBLE_TYPE_NAME),
                LiteralValue::CurPos => self.ensure_base_type(cx, span, FLOAT_TYPE_NAME),
                LiteralValue::Null => self.ensure_base_type(cx, span, ANY_TYPE_NAME),
            },

            Value::BinOp(it) => {
                self.visit_value(cx, &mut it.lhs);
                self.visit_value(cx, &mut it.rhs);

                if let Some(func) = self.find_binop_func(cx, it) {
                    let ty = func.data.ret.clone();
                    it.resolved = Some(func);
                    ty
                } else {
                    None
                }
            }

            Value::DslLiteral(it) => {
                self.visit_value(cx, &mut it.value);

                if let Some(func) = self.find_dsl_func(cx, it) {
                    let ty = func.data.ret.clone();
                    it.resolved = Some(func);
                    ty
                } else {
                    None
                }
            }

            Value::Call(it) => {
                for arg in &mut it.args {
                    self.visit_value(cx, arg);
                }

                if let Some(target) = &mut it.target {
                    self.visit_value(cx, target);
                }

                let func = self.resolve_call(cx, it);

                if let Some(func) = func {
                    let ty = func.data.ret.clone();

                    it.resolved = Some(func);

                    ty
                } else {
                    None
                }
            }

            Value::ValueRef(it) => {
                self.visit_value(cx, &mut it.root);

                let mut ty = self.resolve_ty(cx, &mut it.root);
                let mut i = 0;

                while let Some(found) = &ty {
                    if i >= it.path.len() {
                        break;
                    }

                    ty = self.find_nested_struct_field_type(cx, found, it.path[i]);

                    i += 1;
                }

                ty
            }

            Value::VarRef(it) => {
                if let Some(var) = self.find_var(cx, it.name) {
                    it.resolved = var.data.ty.clone().map(|it| Remote {
                        module: var.module,
                        span: var.span,
                        data: it,
                    });

                    var.data.ty
                } else if let Some(var) = self.find_const(cx, it.name) {
                    it.resolved = Some(Remote {
                        module: var.module,
                        span: var.span,
                        data: var.data.ty.clone(),
                    });

                    Some(var.data.ty)
                } else if let Some(obj) = self.find_objective(cx, it.name) {
                    let ty = self.ensure_base_type(cx, it.span, OBJECTIVE_TYPE_NAME);

                    it.resolved = ty.clone().map(|ty| Remote {
                        module: obj.module,
                        span: obj.span,
                        data: ty,
                    });

                    ty
                } else {
                    None
                }
            }

            Value::Unary(it) => {
                self.visit_value(cx, &mut it.value);

                if let Some(func) = self.find_unary_func(cx, it) {
                    let ty = func.data.ret.clone();
                    it.resolved = Some(func);
                    ty
                } else {
                    None
                }
            }

            Value::NbtLiteral(it) => {
                for (_, val) in &mut it.values {
                    self.visit_value(cx, val);
                }

                self.ensure_base_type(cx, span, NBT_TYPE_NAME)
            }

            Value::ArrayLiteral(it) => {
                for value in &mut it.values {
                    self.visit_value(cx, value);
                }

                if let Some(first) = it.values.first_mut() {
                    let mut ty = self.resolve_ty(cx, first);

                    if let Some(ty) = &mut ty {
                        if let Some(res) = &mut ty.resolved {
                            res.array = ArrayKind::Sized;
                        }
                    }

                    ty
                } else {
                    None
                }
            }

            Value::TypedValue(it) => Some(it.ty.clone()),
        };

        ty
    }
}

impl<'a, 'visit> ValueVisitor<'a, 'visit> for TypeInference {
    fn visit_value(&mut self, cx: &mut VisitCx<'a, 'visit>, node: &mut Value<'a>) {
        let span = node.span();
        let ty = self.resolve_ty(cx, node);

        if let Some(ty) = ty {
            *node = Value::TypedValue(TypedValue {
                value: Box::new(node.clone()),
                span,
                ty,
            });
        }
    }
}
