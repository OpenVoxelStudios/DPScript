use itertools::Itertools;
use miette::NamedSource;

use crate::dpscript::{
    ast::{
        block::BlockNode,
        constant::ConstantNode,
        enums::EnumNode,
        func::{FuncFlags, FunctionNode},
        import::ImportNode,
        node::Node,
        objective::ObjectiveNode,
        var::{VarInfo, VarNode},
    },
    ty::TypeRef,
};
use std::{collections::BTreeMap, sync::Arc};

/// The type of an exported object.
/// These will have all their bodies cleared, since it's unnecessary when just being used for type checking.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum ExportType {
    Constant(ConstantNode),
    Objective(ObjectiveNode),
    Function(FunctionNode),
    Enum(EnumNode),
    // Blocks aren't exported since they're just private functions called based on Minecraft's "hooks" (tags)
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct AST {
    /// The module's name (a::b::c)
    pub module: String,

    /// The source code of the module.
    #[serde(skip)]
    pub code: NamedSource<String>,
    pub imports: Vec<ImportNode>,
    pub constants: BTreeMap<String, ConstantNode>,
    pub objectives: BTreeMap<String, ObjectiveNode>,
    pub functions: BTreeMap<String, FunctionNode>,
    pub blocks: Vec<BlockNode>,
    pub enums: BTreeMap<String, EnumNode>,

    /// All the symbols the module exports.
    pub exports: BTreeMap<String, ExportType>,

    /// All the nodes, including the ones above.
    pub nodes: Vec<Node>,

    /// A map of types to user-defined instance methods.
    pub instance_funcs: BTreeMap<String, BTreeMap<String, FunctionNode>>,

    /// A map of types to their fields' types.
    pub fields: BTreeMap<TypeRef, BTreeMap<String, TypeRef>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Scope {
    #[serde(skip)]
    pub module: Arc<AST>,

    /// A map of local variables to their value types.
    pub locals: BTreeMap<String, VarNode>,
}

macro_rules! only {
    ($list: ident: $type: ident) => {
        $list
            .iter()
            .filter_map(|it| match it {
                Node::$type(value) => Some(value.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
    };

    (m; $list: ident: $type: ident) => {
        $list
            .iter()
            .filter_map(|it| match it {
                Node::$type(value) => Some((value.name.clone(), value.clone())),
                _ => None,
            })
            .collect::<BTreeMap<_, _>>()
    };
}

impl AST {
    pub fn new(module: String, code: NamedSource<String>, nodes: Vec<Node>) -> Self {
        Self {
            module,
            code,
            exports: collect_exports(&nodes),
            blocks: only!(nodes: Block),
            constants: only!(m; nodes: Constant),
            enums: only!(m; nodes: Enum),
            functions: only!(m; nodes: Function),
            imports: only!(nodes: Import),
            objectives: only!(m; nodes: Objective),
            fields: BTreeMap::new(), // TODO

            // My little baby abomination... I'm so proud of it :) (*sobbing*)
            instance_funcs: only!(nodes: Function)
                .into_iter()
                .filter_map(|it| it.receiver.clone().map(|r| (r, (it.name.clone(), it))))
                .into_group_map()
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().collect::<BTreeMap<_, _>>()))
                .collect::<BTreeMap<_, _>>(),

            nodes,
        }
    }
}

impl Scope {
    pub fn lookup(&self, var: impl AsRef<str>) -> Option<&dyn VarInfo> {
        match self.locals.get(&var.as_ref().to_string()) {
            Some(it) => Some(it),
            None => self
                .module
                .constants
                .get(&var.as_ref().to_string())
                .map(|it| it as &dyn VarInfo)
                .or(self
                    .module
                    .objectives
                    .get(&var.as_ref().to_string())
                    .map(|it| it as &dyn VarInfo)),
        }
    }
}

pub fn collect_exports(nodes: &Vec<Node>) -> BTreeMap<String, ExportType> {
    let mut map = BTreeMap::new();

    for node in nodes {
        match node {
            Node::Constant(var) => {
                if var.is_public {
                    map.insert(var.name.clone(), ExportType::Constant(var.clone()))
                } else {
                    None
                }
            }

            Node::Objective(obj) => {
                if obj.is_public {
                    map.insert(obj.name.clone(), ExportType::Objective(obj.clone()))
                } else {
                    None
                }
            }

            Node::Function(func) => {
                if func.flags.contains(FuncFlags::Public) {
                    map.insert(func.name.clone(), ExportType::Function(func.clone()))
                } else {
                    None
                }
            }

            Node::Enum(en) => map.insert(en.name.clone(), ExportType::Enum(en.clone())),

            _ => None,
        };
    }

    map
}
