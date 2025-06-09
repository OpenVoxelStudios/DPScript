// TODO: Split this into multiple files, maybe in `validator::types`?

use crate::{
    BuiltInTypes, Call, CheckerContext, Literal, Module, Node, Objective, Operation, Result, Type,
    TypeKind, ValidatorError, Variable,
};

use super::ExportNode;

impl Objective {
    pub fn get_type(&self) -> Type {
        Type {
            kind: TypeKind::Objective,
            span: self.span,
        }
    }
}

impl Variable {
    pub fn get_type(&self, module: &Module, cx: &mut CheckerContext) -> Result<Type> {
        if let Some(ty) = self.ty.clone() {
            Ok(ty)
        } else {
            self.value.clone().unwrap().get_type(module, cx)
        }
    }
}

impl Call {
    pub fn get_type(&self, module: &Module, cx: &mut CheckerContext) -> Result<Type> {
        let funcs = module.funcs();
        let imports = module.imported_objects()?;
        let refs = cx.get_refs()?;

        if let Some((name, span)) = self.parent.clone() {
            let mut parent = None;

            for (item, it) in refs {
                if name == item {
                    parent = Some(it);
                    break;
                }
            }

            if parent.is_none() {
                return Err(ValidatorError {
                    src: module.source.clone(),
                    at: span,
                    err: format!("Cannot resolve object: {}", name),
                }
                .into());
            }

            let parent = parent.unwrap();
            let base_ty = BuiltInTypes::from(parent.get_type(module, cx)?);
            let methods = base_ty.methods();
            let mut method = None;

            for (name, args, ret) in methods {
                if name == self.function.0 {
                    method = Some((name, args, ret));
                    break;
                }
            }

            if method.is_none() {
                return Err(ValidatorError {
                    src: module.source.clone(),
                    at: self.function.1,
                    err: format!(
                        "Object {} does not define a method: {}",
                        name, self.function.0
                    ),
                }
                .into());
            }

            let (_, _, ret) = method.unwrap();

            return Ok(ret.map(|v| v.as_type(self.span)).unwrap_or(Type {
                kind: TypeKind::Void,
                span: self.span,
            }));
        }

        for func in funcs {
            if func.name.0 == self.function.0 {
                return Ok(func.ret.unwrap_or(Type {
                    kind: TypeKind::Void,
                    span: func.span,
                }));
            }
        }

        for import in imports {
            if let ExportNode::Function(func) = import.export {
                if func.name.0 == self.function.0 {
                    return Ok(func.ret.unwrap_or(Type {
                        kind: TypeKind::Void,
                        span: func.span,
                    }));
                }
            }
        }

        Err(ValidatorError {
            src: module.source.clone(),
            at: self.function.1,
            err: format!("Cannot resolve function: {}", self.function.0),
        }
        .into())
    }
}

impl Literal {
    pub fn get_type(&self, module: &Module, cx: &mut CheckerContext) -> Result<Type> {
        let src = module.source.clone();

        let kind = match self.clone() {
            Literal::Int(_) => TypeKind::Int,
            Literal::Float(_) => TypeKind::Float,
            Literal::Double(_) => TypeKind::Double,
            Literal::Bool(_) => TypeKind::Bool,
            Literal::String(_) => TypeKind::String,
            Literal::Component(_) => TypeKind::Component,
            Literal::Identifier(_) => TypeKind::Identifier,
            Literal::Path(_) => TypeKind::NBTPath,
            Literal::Store(_) => TypeKind::Store,
            Literal::Entity(_) => TypeKind::Entity,
            Literal::Selector(_) => TypeKind::Selector,
            Literal::EnumValue((name, _), _) => TypeKind::Ident(name),
            Literal::Nbt(_) => TypeKind::NBT,
            Literal::Pos3(_, _, _, _) => TypeKind::Pos3,

            Literal::Array((arr, span)) => {
                let mut items = Vec::new();

                for item in arr {
                    items.push((item.get_type(module, cx)?, item.span()));
                }

                if items.is_empty() {
                    return Err(ValidatorError {
                        src,
                        at: span,
                        err: "Cannot infer type for an array with no elements!".into(),
                    }
                    .into());
                }

                let ty_full = items.first().unwrap().0.clone();
                let ty = ty_full.kind.clone();

                for (item, span) in items {
                    if (item.kind.is_numeric() || item.kind == TypeKind::String)
                        && ty == TypeKind::Component
                    {
                        // In this case, we'll just embed the data in the `extra` part of the component
                        continue;
                    }

                    if !item.kind.is_compatible(&ty) {
                        return Err(ValidatorError {
                            src,
                            at: span,
                            err: format!(
                                "Item type is ({}) incompatible with the rest of the array ({})!",
                                item.kind, ty
                            ),
                        }
                        .into());
                    }
                }

                TypeKind::Array(Box::new(ty_full))
            }
        };

        Ok(Type {
            span: self.span(),
            kind,
        })
    }
}

impl Operation {
    pub fn get_type(&self, module: &Module, cx: &mut CheckerContext) -> Result<Type> {
        let kind = self.kind.name();
        let lhs_ty = self.lhs.get_type(&module, cx)?;
        let rhs_ty = self.rhs.get_type(&module, cx)?;
        let builtin_lhs = BuiltInTypes::from(lhs_ty);
        let builtin_rhs = BuiltInTypes::from(rhs_ty);
        let lhs_methods = builtin_lhs.methods();
        let rhs_methods = builtin_rhs.methods();

        let lhs_method = lhs_methods
            .iter()
            .cloned()
            .find(|(name, _, _)| name == &kind);

        let rhs_method = rhs_methods
            .iter()
            .cloned()
            .find(|(name, _, _)| name == &kind);

        if lhs_method.is_none() && rhs_method.is_none() {
            return Err(ValidatorError {
                src: module.source(),
                at: self.lhs.span(),
                err: format!("Operands do not support the method: {}", kind),
            }
            .into());
        }

        let (_, _, ret) = lhs_method.unwrap_or_else(|| rhs_method.unwrap());

        Ok(ret.unwrap_or(BuiltInTypes::Void).as_type(self.span))
    }
}

impl Node {
    pub fn get_type(&self, module: &Module, cx: &mut CheckerContext) -> Result<Type> {
        let src = module.source.clone();

        let kind = match self.clone() {
            Self::Module(_)
            | Self::Function(_)
            | Self::Block(_)
            | Self::Import(_)
            | Self::Export(_)
            | Self::Loop(_)
            | Self::Enum(_)
            | Self::Return(_)
            | Self::Conditional(_)
            | Self::Subroutine(_)
            | Self::Goto(_)
            | Self::Objective(_)
            | Self::Assign(_) => {
                return Err(ValidatorError {
                    src,
                    at: self.span(),
                    err: "Nodes that aren't values should never be types!".into(),
                }
                .into());
            }

            Self::Ident((id, span)) => {
                let refs = cx.get_refs()?;

                if let Some(it) = refs.get(&id) {
                    it.get_type(module, cx)?.kind
                } else {
                    return Err(ValidatorError {
                        src,
                        at: span,
                        err: format!("Cannot resolve reference: {}", id),
                    }
                    .into());
                }
            }

            Self::Literal(lit) => lit.get_type(module, cx)?.kind,
            Self::Operation(op) => op.get_type(module, cx)?.kind,
            Self::Variable(var) => var.get_type(module, cx)?.kind,
            Self::Call(call) => call.get_type(module, cx)?.kind,
        };

        Ok(Type {
            span: self.span(),
            kind,
        })
    }
}
