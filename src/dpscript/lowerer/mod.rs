mod ctx;
mod node;
mod remap;

pub use ctx::*;
pub use node::*;
pub use remap::*;

use super::ExportNode;
use crate::{
    AST, Error, IRAst, IRCall, IRDefinition, IRFunction, IRNode, IRTag, Result, VariableAlias,
};

#[derive(Debug, Clone)]
pub struct Lowerer {
    pub ns: String,
    pub ast: AST,
    pub lowered: Option<IRAst>,
}

impl Lowerer {
    pub fn new(ns: impl AsRef<str>, ast: AST) -> Self {
        Self {
            ns: ns.as_ref().into(),
            ast,
            lowered: None,
        }
    }

    pub fn run(mut self) -> Result<Self> {
        self.ast.indexed = false;
        self.ast.cached = false;
        self.ast.cache_values()?;

        let mut cx = self.ast.create_checker_context()?;
        let mut lcx = LoweringContext::new(&self.ns);
        let mut nodes = Vec::new();
        let mut modules = cx.modules.clone();

        nodes.push(IRNode::Definition(IRDefinition::VariableAlias(
            VariableAlias {
                name: "__RETURN_VAL__".into(),
                store: "dpscript:core/funcs".into(),
                path: "return_value".into(),
            },
        )));

        for (_, item) in &mut modules {
            nodes.extend(item.lower(&mut cx, &mut lcx)?);
        }

        for (name, item) in &mut modules {
            let exports = item.get_exports(&cx)?;

            for item in exports {
                if let ExportNode::Function(func) = item.node {
                    let alias = func.ir_name(&lcx.namespace, name);
                    let real = func.ir_name(&lcx.namespace, item.module);

                    if alias != real
                        && !cx.funcs.contains_key(&alias)
                        && !func.is_compiler
                        && !func.is_facade
                        && !func.is_inline
                    {
                        nodes.push(IRNode::Function(IRFunction {
                            id: alias,
                            body: vec![IRNode::Call(IRCall { function: real })],
                        }));
                    }
                }
            }
        }

        nodes.push(IRNode::Tag(IRTag {
            name: "minecraft:tags/functions/load".into(),
            entries: lcx.init_names,
        }));

        nodes.push(IRNode::Tag(IRTag {
            name: "minecraft:tags/functions/tick".into(),
            entries: lcx.tick_names,
        }));

        self.lowered = Some(IRAst {
            nodes,
            indexed: false,
            funcs: None,
            tags: None,
            vars: None,
        });

        Ok(self)
    }

    pub fn get_code(&self) -> Result<String> {
        if let Some(it) = &self.lowered {
            Ok(it.serialize_nodes())
        } else {
            Err(Error::Basic("The lowerer has not been run yet!".into()))
        }
    }
}
