use super::Lowerable;
use crate::{
    CheckerContext, IRArgumentOperation, IRBlock, IRDefinition, IRExecute, IRGetArgument, IRNode,
    InsertHelper, Literal, Loop, LowererError, LoweringContext, Node, Reference, Result, TypeKind,
    VariableAlias,
};

impl Lowerable for Loop {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.body.is_empty() {
            return Ok(Vec::new());
        }

        let join_block = format!("block{}", lcx.blocks);
        lcx.blocks += 1;

        let mut nodes = Vec::new();
        let refs = cx.get_refs()?;

        let Some(it) = refs.get(&self.array.0) else {
            return Err(LowererError {
                src: cx.get_source(),
                at: self.array.1,
                err: format!("Could not resolve reference: {}", self.array.0),
            }
            .into());
        };

        cx.cur_loops.push(self.clone());

        match it {
            Reference::Variable(var) => {
                let ty = var.get_type(&cx.current_module(), cx)?;
                let mut body = Vec::new();

                if ty.kind == TypeKind::Selector {
                    for item in &mut self.body {
                        body.extend(item.lower(cx, lcx)?);
                    }

                    body.push(IRNode::Goto(join_block.clone()));

                    let id = format!("block{}", lcx.blocks);
                    lcx.blocks += 1;

                    lcx.block_nodes.push(IRBlock {
                        id: id.clone(),
                        body,
                    });

                    nodes.push(IRNode::Execute(IRExecute {
                        block: id,
                        selector: match *var.value.clone().unwrap() {
                            Node::Literal(lit) => match lit {
                                Literal::Selector((sel, _)) => sel,

                                _ => {
                                    return Err(LowererError {
                                        src: cx.get_source(),
                                        at: lit.span(),
                                        err: "Could not get a value out of a literal!".into(),
                                    }
                                    .into());
                                }
                            },

                            it => {
                                return Err(LowererError {
                                    src: cx.get_source(),
                                    at: it.span(),
                                    err: "Could not get a value out of a node!".into(),
                                }
                                .into());
                            }
                        },
                    }));
                } else {
                    let var = if self.var_name.0.starts_with("__var_")
                        || self.var_name.0 == "__RETURN_VAL__"
                    {
                        self.var_name.0.clone()
                    } else {
                        format!("__var_{}", self.var_name.0)
                    };

                    lcx.defs.insert_if_absent(
                        var.clone(),
                        IRDefinition::VariableAlias(VariableAlias {
                            name: var.clone(),
                            store: "dpscript:core/vars".into(),
                            path: var.clone(),
                        }),
                    );

                    body.push(IRNode::Argument(IRArgumentOperation::Get(IRGetArgument {
                        index: 0,
                        var: var.clone(),
                    })));

                    for item in &mut self.body {
                        body.extend(item.lower(cx, lcx)?);
                    }

                    body.push(IRNode::Goto(join_block.clone()));

                    todo!("For each loops")
                }
            }

            _ => panic!("[Achievement Unlocked] How did we get here?"),
        }

        lcx.join_block = Some(IRBlock {
            id: join_block,
            body: Vec::new(),
        });

        lcx.join_dirty = true;
        cx.cur_loops.pop();

        Ok(nodes)
    }
}
