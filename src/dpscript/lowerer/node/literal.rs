use uuid::Uuid;

use super::{Lowerable, Valued};
use crate::{
    AddDataOperation, CheckerContext, IRDataOperation, IRDefinition, IRLiteral, IRNode,
    InsertHelper, Literal, LoweringContext, Reference, Result, VariableAlias,
};

impl Valued for Literal {
    fn get_value(
        &mut self,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
        nodes: &mut Vec<IRNode>,
    ) -> Result<IRNode> {
        Ok(match self {
            Self::EnumValue(_, (v, _)) => IRNode::Literal(IRLiteral::String(v.to_lowercase())),
            Self::Float((v, _)) => IRNode::Literal(IRLiteral::String(format!("{}f", v))),
            Self::Int((i, _)) => IRNode::Literal(IRLiteral::String(i.to_string())),
            Self::Double((d, _)) => IRNode::Literal(IRLiteral::String(format!("{}d", d))),
            Self::Nbt((v, _)) => IRNode::Literal(IRLiteral::String(serde_json::to_string(&v)?)),
            Self::Path((p, _)) => IRNode::Literal(IRLiteral::String(p.clone())),
            Self::Store((s, _)) => IRNode::Literal(IRLiteral::String(s.clone())),
            Self::String((s, _)) => IRNode::Literal(IRLiteral::String(format!("\"{}\"", s))),

            Self::Bool((b, _)) => {
                IRNode::Literal(IRLiteral::String((if *b { "1b" } else { "0b" }).into()))
            }

            Self::Component((c, _)) => {
                IRNode::Literal(IRLiteral::String(format!("{{\"text\": \"{}\"}}", c)))
            }

            Self::Entity((e, _)) => {
                IRNode::Literal(IRLiteral::String(format!("{{\"selector\": \"{}\"}}", e)))
            }

            Self::Selector((s, _)) => {
                IRNode::Literal(IRLiteral::String(format!("{{\"selector\": \"{}\"}}", s)))
            }

            Self::Identifier((id, _)) => {
                let refs = cx.get_refs()?;

                if let Some(it) = refs.get(id) {
                    if let Reference::Variable(var) = it {
                        if var.is_const {
                            return Ok(var.value.clone().unwrap().get_value(cx, lcx, nodes)?);
                        }
                    }
                }

                IRNode::Reference(id.clone())
            }

            Self::Array((vals, _)) => {
                let var = format!("__tmp_var_{}", Uuid::new_v4().to_string().replace("-", "_"));

                lcx.defs.insert_if_absent(
                    var.clone(),
                    IRDefinition::VariableAlias(VariableAlias {
                        name: var.clone(),
                        store: "dpscript:core/vars".into(),
                        path: var.clone(),
                    }),
                );

                nodes.push(IRNode::DataOperation(IRDataOperation::Set(
                    AddDataOperation {
                        var: var.clone(),
                        value: Box::new(IRNode::Literal(IRLiteral::String("[]".into()))),
                    },
                )));

                for item in vals {
                    let add = item.get_value(cx, lcx, nodes)?;

                    nodes.push(IRNode::DataOperation(IRDataOperation::Append(
                        AddDataOperation {
                            var: var.clone(),
                            value: Box::new(add),
                        },
                    )));
                }

                IRNode::Reference(var)
            }

            Self::Pos3(x, y, z, _) => {
                todo!()
            }
        })
    }
}

impl Lowerable for Literal {
    fn lower(
        &mut self,
        _cx: &mut CheckerContext,
        _lcx: &mut LoweringContext,
    ) -> Result<Vec<IRNode>> {
        Ok(Vec::new())
    }
}
