use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::{
    Result,
    dpscript::ast::{
        ast::{AST, ExportType},
        literal::LiteralNode,
        node::Node,
    },
    mc::{
        Command, Function, Literal, OptionLiteral, ScoreboardCommand, ScoreboardObjectivesCommand,
    },
    util::Identifier,
};

use super::ast::literal::LiteralData;

pub struct CodeGenerator {
    pub out_dir: PathBuf,

    /// The current AST we are compiling.
    pub ast: AST,

    /// The resolved imports.
    pub imports: HashMap<String, ExportType>,

    /// A map available modules.
    pub modules: Arc<HashMap<String, AST>>,
}

impl CodeGenerator {
    pub fn new(
        out_dir: PathBuf,
        ast: AST,
        imports: HashMap<String, ExportType>,
        modules: Arc<HashMap<String, AST>>,
    ) -> Self {
        Self {
            out_dir,
            ast,
            modules,
            imports,
        }
    }

    pub fn run(self) -> Result<()> {
        let mut funcs = Vec::new();

        let mut global_init = Function::new(Identifier::new(
            &self.ast.namespace,
            format!(
                "zzz/{}/funcs/_dps_global_init",
                self.ast.module.replace("::", "/")
            ),
        ));

        global_init.always_write = false;

        for (_, item) in &self.ast.scope.objectives {
            let cmd = Command::Scoreboard {
                inner: ScoreboardCommand::Objectives {
                    inner: ScoreboardObjectivesCommand::Add {
                        objective: Literal::Inline {
                            inner: item.id.clone(),
                        },

                        criteria: Literal::Inline {
                            inner: item.kind.clone(),
                        },

                        display_name: OptionLiteral::None {},
                    },
                },
            };

            global_init.commands.push(cmd);
        }

        for block in &self.ast.scope.blocks {
            let mut func = Function::new(block.ident.clone());

            // TODO: Respect attrs and stuff?

            func.commands = self.generate_code(&block.body);

            funcs.push(func);
        }

        funcs.push(global_init);

        for func in funcs {
            func.write(&self.out_dir)?;
        }

        Ok(())
    }

    pub fn generate_code(&self, body: &Vec<Node>) -> Vec<Command> {
        let mut cmds = Vec::new();

        for node in body {
            cmds.extend(self.codegen(node));
        }

        cmds
    }

    pub fn codegen(&self, node: &Node) -> Vec<Command> {
        let mut cmds = Vec::new();

        match node {
            Node::Constant(_) | Node::Enum(_) | Node::Field(_) => {}
            Node::Function(function_node) => todo!(),
            Node::UnaryOp(unary_op_node) => todo!(),
            Node::BinaryOp(binary_op_node) => todo!(),
            Node::Variable(var_node) => todo!(),
            Node::Block(block_node) => todo!(),
            Node::Literal(literal_node) => todo!(),
            Node::Call(call_node) => todo!(),
            Node::Conditional(conditional_node) => todo!(),
            Node::Ident(ident_node) => todo!(),
            Node::Loop(loop_node) => todo!(),
            Node::Objective(objective_node) => todo!(),
            Node::Import(import_node) => todo!(),
            Node::Return(return_node) => todo!(),
            Node::Special(special_node) => todo!(),
            Node::At(at_node) => todo!(),
        };

        cmds
    }

    pub fn codegen_literal(&self, node: &LiteralNode, cmds: &mut Vec<Command>) -> Literal {
        match &node.data {
            LiteralData::String(it) => Literal::Inline { inner: it.clone() },
            LiteralData::Int(it) => Literal::Inline {
                inner: it.to_string(),
            },
            LiteralData::Float(it) => Literal::Inline {
                inner: it.to_string(),
            },
            LiteralData::Double(it) => Literal::Inline {
                inner: it.to_string(),
            },
            LiteralData::Bool(it) => Literal::Inline {
                inner: it.to_string(),
            },
            LiteralData::Array(nodes) => todo!(),
            LiteralData::Nbt(nbt_value) => todo!(),
        }
    }
}
