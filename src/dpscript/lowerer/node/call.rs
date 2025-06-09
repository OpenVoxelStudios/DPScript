use super::{Lowerable, Valued};
use crate::{
    AttributeValue, BuiltInTypes, Call, CheckerContext, ExportNode, Function, IRArgumentOperation,
    IRCall, IRCommand, IRLiteral, IRNode, IRSetArgument, LowererError, LoweringContext, Node,
    Remap, Result, TypeKind,
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::json;

pub const SELECTOR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?m)\{"selector": "([^"]+)"\}"#).unwrap());

impl Call {
    fn handle_inline(
        &mut self,
        mut func: Function,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
    ) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();
        let args = self.args.clone();
        let real_args = func.args.clone();

        for (i, arg) in args.iter().cloned().enumerate() {
            let real = real_args.get(i).unwrap();

            if let Node::Ident(id) = arg {
                // TODO: Handle replacing IDs with literal values
                func = func.remap_name(&real.name.0, &id.0);
            }
        }

        for mut item in func.body {
            nodes.extend(item.lower(cx, lcx)?);
        }

        Ok(nodes)
    }

    fn handle_facade(
        &mut self,
        func: Function,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
    ) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        let Some(syntax) = func.attrs.iter().find(|v| v.name.0 == "cmd") else {
            return Err(LowererError {
                src: cx.get_source(),
                at: func.span,
                err: "Facade function is missing #[cmd()] attribute!".into(),
            }
            .into());
        };

        let AttributeValue::String((it, _)) = syntax.value.clone() else {
            return Err(LowererError {
                src: cx.get_source(),
                at: syntax.span,
                err: format!("Unexpected value: {:?}", syntax.value),
            }
            .into());
        };

        let parts = it
            .split_inclusive('%')
            .map(String::from)
            .collect::<Vec<_>>();

        let mut func_args = func.args.clone();
        let mut call_args = self.args.clone();
        let mut args = Vec::new();

        for mut item in parts {
            if item == "%" {
                let mut arg = call_args.remove(0);
                let real = func_args.remove(0);
                let value = arg.get_value(cx, lcx, &mut nodes)?;

                if real.ty.kind == TypeKind::Selector || real.ty.kind == TypeKind::Entity {
                    if let IRNode::Literal(lit) = &value {
                        if let IRLiteral::String(val) = lit {
                            if SELECTOR_REGEX.is_match(val) {
                                args.push(IRNode::Literal(
                                    SELECTOR_REGEX.replace(val, "$1").into(),
                                ));

                                continue;
                            }
                        }
                    }
                }

                if let IRNode::Reference(it) = &value {
                    if !it.starts_with('"') && self.function.0 == "tellraw" {
                        // NBT interpret, yadda yadda
                        let data = json!({
                            "storage": "dpscript:core/vars",
                            "nbt": it,
                            "interpret": true,
                        });

                        args.push(IRNode::Literal(IRLiteral::String(serde_json::to_string(
                            &data,
                        )?)));

                        continue;
                    }
                }

                args.push(value);
            } else {
                if item.ends_with('%') {
                    item = item.trim_end_matches('%').into();
                }

                if item.is_empty() || item.trim().is_empty() {
                    continue;
                }

                args.push(IRNode::Literal(item.trim().into()));
            }
        }

        nodes.push(IRCommand { cmd: args }.into());

        Ok(nodes)
    }

    fn handle_compiler(
        &mut self,
        _func: Function,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
    ) -> Result<Vec<IRNode>> {
        match self.function.0.as_str() {
            "set_data" => {
                let args = self.args.clone();
                let mut arg0 = args.get(0).cloned().unwrap();
                let mut arg1 = args.get(1).cloned().unwrap();
                let mut arg2 = args.get(2).cloned().unwrap();

                let store = match arg0.get_value(cx, lcx, &mut Vec::new())? {
                    IRNode::Literal(it) => match it {
                        IRLiteral::String(it) => IRLiteral::String(it),
                        IRLiteral::StoreOf(it) => IRLiteral::StoreOf(it),

                        _ => {
                            return Err(LowererError {
                                src: cx.get_source(),
                                at: self.span,
                                err: "Expected a Store for argument 1!".into(),
                            }
                            .into());
                        }
                    },

                    _ => {
                        return Err(LowererError {
                            src: cx.get_source(),
                            at: self.span,
                            err: "Expected a Store for argument 1!".into(),
                        }
                        .into());
                    }
                };

                let path = match arg1.get_value(cx, lcx, &mut Vec::new())? {
                    IRNode::Literal(it) => match it {
                        IRLiteral::String(it) => IRLiteral::String(it),
                        IRLiteral::PathOf(it) => IRLiteral::PathOf(it),

                        _ => {
                            return Err(LowererError {
                                src: cx.get_source(),
                                at: self.span,
                                err: "Expected an NBTPath for argument 2!".into(),
                            }
                            .into());
                        }
                    },

                    _ => {
                        return Err(LowererError {
                            src: cx.get_source(),
                            at: self.span,
                            err: "Expected an NBTPath for argument 2!".into(),
                        }
                        .into());
                    }
                };

                match arg2.get_value(cx, lcx, &mut Vec::new())? {
                    IRNode::Literal(it) => match it {
                        IRLiteral::String(it) => Ok(vec![IRNode::Command(IRCommand {
                            cmd: vec![
                                IRNode::Literal("data modify storage".into()),
                                IRNode::Literal(store),
                                IRNode::Literal(path),
                                IRNode::Literal("set value".into()),
                                IRNode::Literal(it.into()),
                            ],
                        })]),

                        _ => {
                            return Err(LowererError {
                                src: cx.get_source(),
                                at: self.span,
                                err: "Expected a literal for argument 2!".into(),
                            }
                            .into());
                        }
                    },

                    IRNode::Reference(it) => Ok(vec![IRNode::Command(IRCommand {
                        cmd: vec![
                            IRNode::Literal("data modify storage".into()),
                            IRNode::Literal(store),
                            IRNode::Literal(path),
                            IRNode::Literal("set from storage".into()),
                            IRNode::Literal(IRLiteral::StoreOf(it.clone())),
                            IRNode::Literal(IRLiteral::PathOf(it.clone())),
                        ],
                    })]),

                    _ => {
                        return Err(LowererError {
                            src: cx.get_source(),
                            at: self.span,
                            err: "Expected a literal or a reference for argument 2!".into(),
                        }
                        .into());
                    }
                }
            }

            _ => {
                return Err(LowererError {
                    src: cx.get_source(),
                    at: self.span,
                    err: format!("{} is not a compiler-generated function!", self.function.0),
                }
                .into());
            }
        }
    }
}

impl Lowerable for Call {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();
        let mut module = cx.modules.get(&lcx.module).unwrap().clone();
        let refs = cx.get_refs()?;
        let objs = module.get_imported_objects(cx)?;
        let mut found = false;

        if let Some((name, span)) = self.parent.clone() {
            let mut parent = None;

            for (item, it) in refs {
                if name == item {
                    parent = Some(it);
                    break;
                }
            }

            if parent.is_none() {
                return Err(LowererError {
                    src: module.source(),
                    at: span,
                    err: format!("Cannot resolve object: {}", name),
                }
                .into());
            }

            let parent = parent.unwrap();
            let base_ty = BuiltInTypes::from(parent.get_type(&module, cx)?);
            let methods = base_ty.methods();
            let mut method = None;

            for (name, args, ret) in methods {
                if name == self.function.0 {
                    method = Some((name, args, ret));
                    break;
                }
            }

            if method.is_none() {
                return Err(LowererError {
                    src: module.source(),
                    at: self.function.1,
                    err: format!(
                        "Object {} does not define a method: {}",
                        name, self.function.0
                    ),
                }
                .into());
            }

            let (id, _, _) = method.unwrap();
            let mut num_args = 0;

            if let Some((parent, _)) = self.parent.clone() {
                nodes.push(IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
                    index: 0,
                    value: Box::new(IRNode::Reference(parent)),
                })));

                num_args += 1;
            }

            for arg in &mut self.args {
                let arg = IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
                    index: num_args,
                    value: Box::new(arg.get_value(cx, lcx, &mut nodes)?),
                }));

                num_args += 1;
                nodes.push(arg);
            }

            base_ty.create_ir(id.to_string(), lcx, &mut nodes);

            return Ok(nodes);
        }

        for item in objs {
            if let ExportNode::Function(func) = item.export {
                if func.name.0 == self.function.0 {
                    if func.is_facade {
                        nodes.extend(self.handle_facade(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    if func.is_inline {
                        nodes.extend(self.handle_inline(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    if func.is_compiler {
                        nodes.extend(self.handle_compiler(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    let mut num_args = 0;

                    if let Some((parent, _)) = self.parent.clone() {
                        nodes.push(IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
                            index: 0,
                            value: Box::new(IRNode::Reference(parent)),
                        })));

                        num_args += 1;
                    }

                    for arg in &mut self.args {
                        let arg = IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
                            index: num_args,
                            value: Box::new(arg.get_value(cx, lcx, &mut nodes)?),
                        }));

                        num_args += 1;
                        nodes.push(arg);
                    }

                    let id = func.ir_name(&lcx.namespace, item.module);

                    nodes.push(IRNode::Call(IRCall { function: id }));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for func in module.funcs() {
                if func.name.0 == self.function.0 {
                    if func.is_facade {
                        nodes.extend(self.handle_facade(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    if func.is_inline {
                        nodes.extend(self.handle_inline(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    if func.is_compiler {
                        nodes.extend(self.handle_compiler(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    let mut num_args = 0;

                    if let Some((parent, _)) = self.parent.clone() {
                        nodes.push(IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
                            index: 0,
                            value: Box::new(IRNode::Reference(parent)),
                        })));

                        num_args += 1;
                    }

                    for arg in &mut self.args {
                        let arg = IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
                            index: num_args,
                            value: Box::new(arg.get_value(cx, lcx, &mut nodes)?),
                        }));

                        num_args += 1;
                        nodes.push(arg);
                    }

                    let id = func.ir_name(&lcx.namespace, &lcx.module);

                    nodes.push(IRNode::Call(IRCall { function: id }));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            return Err(LowererError {
                src: cx.get_source(),
                at: self.span,
                err: format!("Could not find function: {}", self.function.0),
            }
            .into());
        }

        Ok(nodes)
    }
}
