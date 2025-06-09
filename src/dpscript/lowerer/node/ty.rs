use uuid::Uuid;

use crate::{
    BuiltInTypes, CopyDataOperation, IRArgumentOperation, IRDataOperation, IRDefinition,
    IRGetArgument, IRNode, LoweringContext, VariableAlias, command,
};

impl BuiltInTypes {
    pub fn create_ir(&self, func: String, lcx: &mut LoweringContext, nodes: &mut Vec<IRNode>) {
        match self {
            Self::Int => {
                let myself = format!("__tmp_var_{}", Uuid::new_v4().to_string().replace("-", "_"));
                let other = format!("__tmp_var_{}", Uuid::new_v4().to_string().replace("-", "_"));

                let myself_score = format!(
                    "__dpscript_score_{}",
                    Uuid::new_v4().to_string().replace("-", "_")
                );

                let other_score = format!(
                    "__dpscript_score_{}",
                    Uuid::new_v4().to_string().replace("-", "_")
                );

                lcx.defs.insert(
                    myself.clone(),
                    IRDefinition::VariableAlias(VariableAlias {
                        name: myself.clone(),
                        store: "dpscript:core/vars".into(),
                        path: myself.clone(),
                    }),
                );

                lcx.defs.insert(
                    other.clone(),
                    IRDefinition::VariableAlias(VariableAlias {
                        name: other.clone(),
                        store: "dpscript:core/vars".into(),
                        path: other.clone(),
                    }),
                );

                lcx.init_nodes.push(
                    command!(
                        "scoreboard",
                        "objectives",
                        "add",
                        myself_score.clone(),
                        "dummy"
                    )
                    .into(),
                );

                lcx.init_nodes.push(
                    command!(
                        "scoreboard",
                        "objectives",
                        "add",
                        other_score.clone(),
                        "dummy"
                    )
                    .into(),
                );

                nodes.push(IRNode::Argument(IRArgumentOperation::Get(IRGetArgument {
                    index: 0,
                    var: myself.clone(),
                })));

                nodes.push(IRNode::Argument(IRArgumentOperation::Get(IRGetArgument {
                    index: 1,
                    var: other.clone(),
                })));

                nodes.push(
                    command!(
                        "execute",
                        "store",
                        "result",
                        "score",
                        "__dpscript_temp",
                        myself_score.clone(),
                        "run",
                        "data",
                        "get",
                        "storage",
                        "dpscript:core/vars",
                        myself.clone()
                    )
                    .into(),
                );

                nodes.push(
                    command!(
                        "execute",
                        "store",
                        "result",
                        "score",
                        "__dpscript_temp",
                        other_score.clone(),
                        "run",
                        "data",
                        "get",
                        "storage",
                        "dpscript:core/vars",
                        other.clone()
                    )
                    .into(),
                );

                match func.as_str() {
                    "add" => {
                        nodes.push(
                            command!(
                                "scoreboard",
                                "players",
                                "operation",
                                "__dpscript_temp",
                                myself_score.clone(),
                                "+=",
                                "__dpscript_temp",
                                other_score.clone()
                            )
                            .into(),
                        );
                    }

                    "sub" => {
                        nodes.push(
                            command!(
                                "scoreboard",
                                "players",
                                "operation",
                                "__dpscript_temp",
                                myself_score.clone(),
                                "-=",
                                "__dpscript_temp",
                                other_score.clone()
                            )
                            .into(),
                        );
                    }

                    "mul" => {
                        nodes.push(
                            command!(
                                "scoreboard",
                                "players",
                                "operation",
                                "__dpscript_temp",
                                myself_score.clone(),
                                "*=",
                                "__dpscript_temp",
                                other_score.clone()
                            )
                            .into(),
                        );
                    }

                    "div" => {
                        nodes.push(
                            command!(
                                "scoreboard",
                                "players",
                                "operation",
                                "__dpscript_temp",
                                myself_score.clone(),
                                "/=",
                                "__dpscript_temp",
                                other_score.clone()
                            )
                            .into(),
                        );
                    }

                    "eq" | "equals" => {
                        nodes.push(
                            command!(
                                "execute",
                                "store",
                                "result",
                                "score",
                                myself_score.clone(),
                                "__dpscript_temp",
                                "run",
                                "execute",
                                "if",
                                "score",
                                "__dpscript_temp",
                                myself_score.clone(),
                                "=",
                                other_score.clone()
                            )
                            .into(),
                        );
                    }

                    "ltEq" | "lessThanEqual" => {
                        nodes.push(
                            command!(
                                "execute",
                                "store",
                                "result",
                                "score",
                                myself_score.clone(),
                                "__dpscript_temp",
                                "run",
                                "execute",
                                "if",
                                "score",
                                "__dpscript_temp",
                                myself_score.clone(),
                                "<=",
                                other_score.clone()
                            )
                            .into(),
                        );
                    }

                    "gtEq" | "greaterThanEqual" => {
                        nodes.push(
                            command!(
                                "execute",
                                "store",
                                "result",
                                "score",
                                myself_score.clone(),
                                "__dpscript_temp",
                                "run",
                                "execute",
                                "if",
                                "score",
                                "__dpscript_temp",
                                myself_score.clone(),
                                ">=",
                                other_score.clone()
                            )
                            .into(),
                        );
                    }

                    "lt" | "lessThan" => {
                        nodes.push(
                            command!(
                                "execute",
                                "store",
                                "result",
                                "score",
                                myself_score.clone(),
                                "__dpscript_temp",
                                "run",
                                "execute",
                                "if",
                                "score",
                                "__dpscript_temp",
                                myself_score.clone(),
                                "<",
                                other_score.clone()
                            )
                            .into(),
                        );
                    }

                    "gt" | "greaterThan" => {
                        nodes.push(
                            command!(
                                "execute",
                                "store",
                                "result",
                                "score",
                                myself_score.clone(),
                                "__dpscript_temp",
                                "run",
                                "execute",
                                "if",
                                "score",
                                "__dpscript_temp",
                                myself_score.clone(),
                                ">",
                                other_score.clone()
                            )
                            .into(),
                        );
                    }

                    _ => panic!("Achievement unlocked: How did we get here?"),
                };

                nodes.push(
                    command!(
                        "execute",
                        "store",
                        "result",
                        "storage",
                        "dpscript:core/vars",
                        myself.clone(),
                        "int",
                        "1",
                        "run",
                        "scoreboard",
                        "players",
                        "get",
                        "__dpscript_temp",
                        myself_score
                    )
                    .into(),
                );

                nodes.push(IRNode::DataOperation(IRDataOperation::Copy(
                    CopyDataOperation {
                        source: myself.clone(),
                        target: "__RETURN_VAL__".into(),
                    },
                )));
            }

            Self::Float => todo!("TODO: Float operations"),

            _ => {}
        }
    }
}
