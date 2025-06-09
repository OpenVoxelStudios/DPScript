use crate::{
    Block, Conditional, Enum, Export, ExportNode, Function, Import, Loop, Module, Objective,
    Reference, Result, Subroutine, Type, TypeKind, ValidatorError, Variable, fake_span,
};
use miette::{NamedSource, SourceOffset, SourceSpan};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Serialize)]
pub struct CheckerContext {
    pub modules: BTreeMap<String, Module>,
    pub imports: BTreeMap<String, Vec<Import>>,
    pub funcs: BTreeMap<String, Vec<Function>>,
    pub vars: BTreeMap<String, Vec<Variable>>,
    pub blocks: BTreeMap<String, Vec<Block>>,
    pub enums: BTreeMap<String, Vec<Enum>>,
    pub objectives: BTreeMap<String, Vec<Objective>>,
    pub exports: BTreeMap<String, Vec<Export>>,

    // State
    pub cur_fn: Option<Function>,
    pub cur_block: Option<Block>,
    pub cur_conds: Vec<Conditional>,
    pub cur_elses: Vec<Conditional>, // These have to be separate
    pub cur_loops: Vec<Loop>,
    pub cur_modules: Vec<Module>,
    pub cur_subroutines: Vec<Subroutine>,
}

impl CheckerContext {
    pub fn current_module(&self) -> Module {
        self.cur_modules.clone().last().unwrap().clone()
    }

    pub fn get_source(&self) -> NamedSource<String> {
        self.cur_modules.clone().last().unwrap().source.clone()
    }

    pub fn get_subroutines(&self) -> HashMap<String, Subroutine> {
        let mut map = HashMap::new();

        if let Some(it) = &self.cur_fn {
            map.extend(it.get_subroutines());
        }

        if let Some(it) = &self.cur_block {
            map.extend(it.get_subroutines());
        }

        for it in &self.cur_conds {
            map.extend(it.get_if_subroutines());
        }

        for it in &self.cur_elses {
            map.extend(it.get_if_subroutines());
        }

        for it in &self.cur_loops {
            map.extend(it.get_subroutines());
        }

        for it in &self.cur_subroutines {
            map.extend(it.get_subroutines());
        }

        map
    }

    // TODO: Stop at a node, so we only get variables defined **before** the node
    pub fn get_refs(&mut self) -> Result<HashMap<String, Reference>> {
        let mut refs: HashMap<String, Reference> = HashMap::new();
        let mut locals: HashMap<String, Variable> = HashMap::new();

        refs.insert(
            "__RETURN_VAL__".into(),
            Reference::Variable(Variable {
                is_arg: true, // It's a builtin, but it functions the same as an arg
                is_const: false,
                is_pub: false,
                name: ("__RETURN_VAL__".into(), fake_span()),
                span: fake_span(),
                ty: Some(Type {
                    kind: TypeKind::Any,
                    span: fake_span(),
                }),
                value: None,
            }),
        );

        if let Some(module) = self.cur_modules.last() {
            refs.extend(
                module
                    .objectives()
                    .iter()
                    .map(|v| (v.name.0.clone(), v.clone().into())),
            );

            refs.extend(
                module
                    .enums()
                    .iter()
                    .map(|v| (v.name.0.clone(), v.clone().into())),
            );

            if let Ok(objs) = module.imported_objects() {
                for item in objs {
                    match item.export {
                        ExportNode::Enum(it) => {
                            refs.insert(it.name.0.clone(), it.into());
                        }

                        ExportNode::Variable(it) => {
                            refs.insert(it.name.0.clone(), it.into());
                        }

                        ExportNode::Objective(it) => {
                            refs.insert(it.name.0.clone(), it.into());
                        }

                        _ => {}
                    }
                }
            }
        }

        if let Some(it) = &mut self.cur_fn {
            for arg in &it.args {
                locals.insert(
                    arg.name.0.clone(),
                    Variable {
                        is_const: false,
                        is_pub: false,
                        is_arg: true,
                        name: arg.name.clone(),
                        span: arg.span,
                        ty: Some(arg.ty.clone()),
                        value: None,
                    },
                );
            }

            for item in it.get_locals() {
                locals.insert(item.name.0.clone(), item);
            }
        }

        if let Some(it) = &mut self.cur_block {
            for item in it.get_locals() {
                locals.insert(item.name.0.clone(), item);
            }
        }

        for it in &mut self.cur_conds {
            for item in it.get_if_locals() {
                locals.insert(item.name.0.clone(), item);
            }
        }

        for it in &mut self.cur_elses {
            for item in it.get_else_locals() {
                locals.insert(item.name.0.clone(), item);
            }
        }

        let mut loops = self.cur_loops.clone();

        for it in &mut loops {
            let arr = it.array.0.clone();
            let arr_span = it.array.1;
            let var_name = it.var_name.clone();

            if let Some(item) = refs.get(&arr) {
                if item.get_type(&self.current_module(), self)?.kind == TypeKind::Selector {
                    locals.insert(
                        arr.clone(),
                        Variable {
                            is_arg: true,
                            is_const: false,
                            is_pub: false,
                            name: var_name.clone(),
                            span: var_name.1,
                            ty: Some(Type {
                                span: SourceSpan::new(
                                    SourceOffset::from_location("Entity", 0, 0),
                                    6,
                                ),
                                kind: TypeKind::Entity,
                            }),
                            value: None,
                        },
                    );
                } else if item.is_array(&self.current_module(), self)? {
                    let Reference::Variable(var) = item else {
                        panic!("Somehow, it was an array and not a var?? How???")
                    };

                    if let Some(ty) = &var.ty {
                        let TypeKind::Array(real) = ty.kind.clone() else {
                            panic!("Achievement Unlocked: How did we get here?")
                        };

                        locals.insert(
                            arr.clone(),
                            Variable {
                                is_arg: true,
                                is_const: false,
                                is_pub: false,
                                name: var_name.clone(),
                                span: var_name.1,
                                ty: Some(*real),
                                value: None,
                            },
                        );
                    } else {
                        return Err(ValidatorError {
                            src: self.current_module().source,
                            at: var.span,
                            err: "Variables used in loops must have a non-inferred type!".into(),
                        }
                        .into());
                    }
                }
            } else if let Some(it) = locals.get(&arr) {
                let ty = it.get_type(&self.current_module(), self)?;

                if ty.kind == TypeKind::Selector {
                    locals.insert(
                        arr.clone(),
                        Variable {
                            is_arg: true,
                            is_const: false,
                            is_pub: false,
                            name: var_name.clone(),
                            span: var_name.1,
                            ty: Some(Type {
                                span: SourceSpan::new(
                                    SourceOffset::from_location("Entity", 0, 0),
                                    6,
                                ),
                                kind: TypeKind::Entity,
                            }),
                            value: None,
                        },
                    );
                } else if let Some(ty) = ty.array_element() {
                    locals.insert(
                        arr.clone(),
                        Variable {
                            is_pub: false,
                            is_const: false,
                            is_arg: true,
                            name: var_name.clone(),
                            ty: Some(ty),
                            value: None,
                            span: var_name.1,
                        },
                    );
                } else {
                    return Err(ValidatorError {
                        src: self.current_module().source,
                        at: arr_span,
                        err: format!("Variable {} is not an array!", arr),
                    }
                    .into());
                }
            }

            for item in it.get_locals() {
                locals.insert(item.name.0.clone(), item);
            }
        }

        self.cur_loops = loops;

        refs.extend(locals.iter().map(|(n, v)| (n.clone(), v.clone().into())));

        Ok(refs)
    }
}
