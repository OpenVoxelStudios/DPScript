use super::{Lowerable, Valued};
use crate::{
    CheckerContext, IRLiteral, IRNode, LowererError, LoweringContext, Node, Reference, Result,
};

impl Valued for Node {
    fn get_value(
        &mut self,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
        nodes: &mut Vec<IRNode>,
    ) -> Result<IRNode> {
        match self {
            Self::Operation(op) => op.get_value(cx, lcx, nodes),
            Self::Literal(lit) => lit.get_value(cx, lcx, nodes),

            Self::Ident((id, _)) => Ok(IRNode::Reference({
                let refs = cx.get_refs()?;

                if let Some(it) = refs.get(id) {
                    if let Reference::Variable(var) = it {
                        if var.is_const {
                            return Ok(var.value.clone().unwrap().get_value(cx, lcx, nodes)?);
                        }
                    } else if let Reference::Objective(obj) = it {
                        return Ok(IRNode::Literal(IRLiteral::String(obj.id.0.clone())));
                    }
                }

                if id.starts_with("__var_") || id == "__RETURN_VAL__" {
                    id.clone()
                } else {
                    format!("__var_{}", id)
                }
            })),

            Self::Call(call) => {
                if call.function.0 == "storeof" {
                    let id = match call.args.first().unwrap() {
                        Node::Ident((id, _)) => id.clone(),

                        it => {
                            return Err(LowererError {
                                src: cx.get_source(),
                                at: it.span(),
                                err: "Expected an identifier!".into(),
                            }
                            .into());
                        }
                    };

                    let id = if id.starts_with("__var_") || id == "__RETURN_VAL__" {
                        id
                    } else {
                        format!("__var_{}", id)
                    };

                    return Ok(IRNode::Literal(IRLiteral::StoreOf(id)));
                }

                if call.function.0 == "keyof" {
                    let id = match call.args.first().unwrap() {
                        Node::Ident((id, _)) => id.clone(),

                        it => {
                            return Err(LowererError {
                                src: cx.get_source(),
                                at: it.span(),
                                err: "Expected an identifier!".into(),
                            }
                            .into());
                        }
                    };

                    let id = if id.starts_with("__var_") || id == "__RETURN_VAL__" {
                        id
                    } else {
                        format!("__var_{}", id)
                    };

                    return Ok(IRNode::Literal(IRLiteral::PathOf(id)));
                }

                nodes.extend(call.lower(cx, lcx)?);

                Ok(IRNode::Reference("__RETURN_VAL__".into()))
            }

            Self::Variable(var) => {
                if var.is_const {
                    var.value.clone().unwrap().get_value(cx, lcx, nodes)
                } else {
                    Ok(IRNode::Reference(
                        if var.name.0.starts_with("__var_") || var.name.0 == "__RETURN_VAL__" {
                            var.name.0.clone()
                        } else {
                            format!("__var_{}", var.name.0)
                        },
                    ))
                }
            }

            _ => Err(LowererError {
                src: cx.get_source(),
                at: self.span(),
                err: "This cannot be used as a value!".into(),
            }
            .into()),
        }
    }
}

impl Lowerable for Node {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        nodes.extend(match self {
            Self::Module(it) => it.lower(cx, lcx)?,
            Self::Literal(it) => it.lower(cx, lcx)?,
            Self::Function(it) => it.lower(cx, lcx)?,
            Self::Variable(it) => it.lower(cx, lcx)?,
            Self::Call(it) => it.lower(cx, lcx)?,
            Self::Operation(it) => it.lower(cx, lcx)?,
            Self::Block(it) => it.lower(cx, lcx)?,
            Self::Loop(it) => it.lower(cx, lcx)?,
            Self::Return(it) => it.lower(cx, lcx)?,
            Self::Objective(it) => it.lower(cx, lcx)?,
            Self::Conditional(it) => it.lower(cx, lcx)?,
            Self::Subroutine(it) => it.lower(cx, lcx)?,
            Self::Goto((it, _)) => vec![IRNode::Goto(format!("__sub_{}", it))],

            // Imports, exports, types, and enums don't get transpiled - they're types.
            _ => Vec::new(),
        });

        Ok(nodes)
    }
}
