use crate::dpscript::{
    ast::{
        block::BlockNode,
        constant::ConstantNode,
        enums::EnumNode,
        func::FunctionNode,
        import::ImportNode,
        node::Node,
        objective::ObjectiveNode,
        var::{VarInfo, VarNode},
    },
    ty::TypeRef,
};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AST {
    pub imports: Vec<ImportNode>,
    pub constants: Vec<ConstantNode>,
    pub objectives: Vec<ObjectiveNode>,
    pub functions: Vec<FunctionNode>,
    pub blocks: Vec<BlockNode>,
    pub enums: Vec<EnumNode>,

    /// All the nodes, including the ones above.
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Module {
    /// A map of globals to their value types.
    /// These globals aren't things like functions, only variables.
    pub globals: BTreeMap<String, ConstantNode>,

    /// A map of function names to nodes (which are their definitions).
    pub funcs: BTreeMap<String, FunctionNode>,

    /// A map of types to user-defined instance methods.
    pub instance_funcs: BTreeMap<TypeRef, BTreeMap<String, FunctionNode>>,

    /// A map of types to their fields' types.
    pub fields: BTreeMap<TypeRef, BTreeMap<String, TypeRef>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Scope {
    #[serde(skip)]
    pub module: Arc<Module>,

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
}

impl AST {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            blocks: only!(nodes: Block),
            constants: only!(nodes: Constant),
            enums: only!(nodes: Enum),
            functions: only!(nodes: Function),
            imports: only!(nodes: Import),
            objectives: only!(nodes: Objective),
            nodes,
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.blocks.extend(other.blocks);
        self.constants.extend(other.constants);
        self.enums.extend(other.enums);
        self.functions.extend(other.functions);
        self.imports.extend(other.imports);
        self.objectives.extend(other.objectives);
        self.nodes.extend(other.nodes);

        Self {
            blocks: self.blocks,
            constants: self.constants,
            enums: self.enums,
            functions: self.functions,
            imports: self.imports,
            objectives: self.objectives,
            nodes: self.nodes,
        }
    }
}

impl Scope {
    pub fn lookup(&self, var: impl AsRef<str>) -> Option<&dyn VarInfo> {
        match self.locals.get(&var.as_ref().to_string()) {
            Some(it) => Some(it),
            None => self
                .module
                .globals
                .get(&var.as_ref().to_string())
                .map(|it| it as &dyn VarInfo),
        }
    }
}
