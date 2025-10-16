use flexstr::SharedStr;
use itertools::Itertools;
use miette::NamedSource;

use crate::dpscript::{
    ast::{
        block::BlockNode,
        constant::ConstantNode,
        enums::EnumNode,
        field::FieldNode,
        func::{FuncFlags, FunctionNode},
        import::ImportNode,
        node::Node,
        objective::ObjectiveNode,
        var::{VarInfo, VarNode},
    },
    ty::{BuiltInType, TypeRef},
};
use std::collections::BTreeMap;

/// The type of an exported object.
/// These will have all their bodies cleared, since it's unnecessary when just being used for type checking.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum ExportType {
    Constant(ConstantNode),
    Objective(ObjectiveNode),
    Function(FunctionNode),
    Enum(EnumNode),
    Field(FieldNode),
    // Blocks aren't exported since they're just private functions called based on Minecraft's "hooks" (tags)
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct AST {
    /// The namespace of the AST.
    pub namespace: SharedStr,

    /// The module's name (a::b::c)
    pub module: SharedStr,

    /// The source code of the module.
    #[serde(skip)]
    pub code: NamedSource<SharedStr>,

    /// All the nodes, including the ones above.
    pub nodes: Vec<Node>,

    pub scope: Scope,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Default)]
pub struct Scope {
    /// The module, function, or block name.
    pub name: SharedStr,

    /// A map of local variables to their value types.
    pub locals: BTreeMap<SharedStr, VarNode>,

    /// A map of types to user-defined instance methods.
    pub instance_funcs: BTreeMap<TypeRef, BTreeMap<SharedStr, FunctionNode>>,

    /// A map of types to their fields' types.
    pub fields: BTreeMap<TypeRef, BTreeMap<SharedStr, FieldNode>>,

    /// All the symbols the module exports.
    pub exports: BTreeMap<SharedStr, ExportType>,

    pub imports: Vec<ImportNode>,
    pub constants: BTreeMap<SharedStr, ConstantNode>,
    pub objectives: BTreeMap<SharedStr, ObjectiveNode>,
    pub functions: BTreeMap<SharedStr, FunctionNode>,
    pub blocks: Vec<BlockNode>,
    pub enums: BTreeMap<SharedStr, EnumNode>,

    /// The parent scopes.
    pub parents: Vec<Scope>,
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
    pub fn new(
        namespace: SharedStr,
        module: SharedStr,
        code: NamedSource<SharedStr>,
        nodes: Vec<Node>,
    ) -> Self {
        Self {
            namespace,
            module: module.clone(),
            code,

            scope: Scope {
                name: module,
                locals: BTreeMap::new(),
                exports: collect_exports(&nodes),
                blocks: only!(nodes: Block),
                constants: only!(m; nodes: Constant),
                enums: only!(m; nodes: Enum),
                functions: only!(m; nodes: Function),
                imports: only!(nodes: Import),
                objectives: only!(m; nodes: Objective),
                parents: Vec::new(),

                fields: only!(nodes: Field)
                    .into_iter()
                    .map(|it| (it.owner.clone(), (it.name.clone(), it)))
                    .into_group_map()
                    .into_iter()
                    .map(|(k, v)| (k, v.into_iter().collect::<BTreeMap<_, _>>()))
                    .collect::<BTreeMap<_, _>>(),

                // My little baby abomination... I'm so proud of it :) (*sobbing*)
                instance_funcs: only!(nodes: Function)
                    .into_iter()
                    .filter_map(|it| it.receiver.clone().map(|r| (r, (it.name.clone(), it))))
                    .into_group_map()
                    .into_iter()
                    .map(|(k, v)| (k, v.into_iter().collect::<BTreeMap<_, _>>()))
                    .collect::<BTreeMap<_, _>>(),
            },

            nodes,
        }
    }
}

impl Scope {
    pub fn new(name: SharedStr, parents: Vec<Scope>) -> Self {
        Self {
            name,
            parents,
            ..Default::default()
        }
    }

    pub fn lookup(&self, var: &SharedStr) -> Option<&dyn VarInfo> {
        match self.locals.get(var) {
            Some(it) => Some(it),
            None => self
                .constants
                .get(var)
                .map(|it| it as &dyn VarInfo)
                .or(self.objectives.get(var).map(|it| it as &dyn VarInfo))
                .or(self.parents.iter().find_map(|it| it.lookup(var))),
        }
    }

    pub fn lookup_fn(&self, name: &SharedStr) -> Option<&FunctionNode> {
        match self.functions.get(name) {
            Some(it) => Some(it),
            None => self.parents.iter().find_map(|it| it.lookup_fn(name)),
        }
    }

    pub fn lookup_inst_fn(&self, ty: &TypeRef, name: &SharedStr) -> Option<&FunctionNode> {
        let res = match self.instance_funcs.get(ty).map(|it| it.get(name)).flatten() {
            Some(it) => Some(it),
            None => self
                .parents
                .iter()
                .find_map(|it| it.lookup_inst_fn(ty, name)),
        };

        match res {
            Some(it) => Some(it),
            _ => {
                if !ty.is_any() {
                    self.lookup_inst_fn(&TypeRef::BuiltIn(BuiltInType::Any), name)
                } else {
                    None
                }
            }
        }
    }

    pub fn lookup_field(&self, ty: &TypeRef, name: &SharedStr) -> Option<&FieldNode> {
        let res = match self.fields.get(ty).map(|it| it.get(name)).flatten() {
            Some(it) => Some(it),
            None => self.parents.iter().find_map(|it| it.lookup_field(ty, name)),
        };

        match res {
            Some(it) => Some(it),
            _ => {
                if !ty.is_any() {
                    self.lookup_field(&TypeRef::BuiltIn(BuiltInType::Any), name)
                } else {
                    None
                }
            }
        }
    }

    pub fn add_local(&mut self, name: SharedStr, value: VarNode) {
        debug!("Scope [{}] -> adding local: {name}", self.name);

        self.locals.insert(name, value);
    }
}

pub fn collect_exports(nodes: &Vec<Node>) -> BTreeMap<SharedStr, ExportType> {
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

            Node::Field(it) => {
                if it.is_public {
                    map.insert(it.name.clone(), ExportType::Field(it.clone()))
                } else {
                    None
                }
            },

            _ => None,
        };
    }

    map
}
