use super::{Lowerable, Valued};
use crate::{
    AddDataOperation, CheckerContext, Conditional, CopyDataOperation, IRBlock, IRCondition,
    IRDataOperation, IRDefinition, IRNode, LoweringContext, Result, VariableAlias, command,
};
use uuid::Uuid;

impl Lowerable for Conditional {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.body.is_empty() && self.else_body.is_empty() {
            return Ok(Vec::new());
        }

        let true_block = format!("block{}", lcx.blocks);
        lcx.blocks += 1;

        let false_block = format!("block{}", lcx.blocks);
        lcx.blocks += 1;

        let join_block = format!("block{}", lcx.blocks);
        lcx.blocks += 1;

        // If blocks

        let mut body = Vec::new();

        for item in &mut self.body {
            body.extend(item.lower(cx, lcx)?);
        }

        body.push(IRNode::Goto(join_block.clone()));

        lcx.block_nodes.push(IRBlock {
            id: true_block.clone(),
            body,
        });

        // Else Block

        let mut body = Vec::new();

        for item in &mut self.else_body {
            body.extend(item.lower(cx, lcx)?);
        }

        body.push(IRNode::Goto(join_block.clone()));

        lcx.block_nodes.push(IRBlock {
            id: false_block.clone(),
            body,
        });

        // Build condition

        let mut nodes = Vec::new();
        let val = self.condition.get_value(cx, lcx, &mut nodes)?;

        let temp = format!("__tmp_var_{}", Uuid::new_v4().to_string().replace("-", "_"));

        let score = format!(
            "__dpscript_score_{}",
            Uuid::new_v4().to_string().replace("-", "_")
        );

        lcx.defs.insert(
            temp.clone(),
            IRDefinition::VariableAlias(VariableAlias {
                name: temp.clone(),
                store: "dpscript:core/vars".into(),
                path: temp.clone(),
            }),
        );

        if let IRNode::Reference(it) = val {
            let node = IRNode::DataOperation(IRDataOperation::Copy(CopyDataOperation {
                source: it,
                target: temp.clone(),
            }));

            nodes.push(node);
        } else {
            let node = IRNode::DataOperation(IRDataOperation::Set(AddDataOperation {
                var: temp.clone(),
                value: Box::new(val),
            }));

            nodes.push(node);
        }

        lcx.init_nodes
            .push(command!("scoreboard", "objectives", "add", score.clone(), "dummy").into());

        nodes.push(
            command!(
                "execute",
                "store",
                "result",
                "score",
                "__dpscript_temp",
                score.clone(),
                "run",
                "data",
                "get",
                "storage",
                "dpscript:core/vars",
                temp.clone()
            )
            .into(),
        );

        nodes.push(IRNode::Condition(IRCondition {
            score,
            if_block: true_block,
            else_block: false_block,
            join_block: join_block.clone(),
        }));

        lcx.join_block = Some(IRBlock {
            id: join_block,
            body: Vec::new(),
        });

        lcx.join_dirty = true;

        Ok(nodes)
    }
}
