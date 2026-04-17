use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use tracing::debug;

use crate::{
    block::BlockNode,
    constant::ConstantNode,
    enums::EnumNode,
    field::FieldNode,
    func::{FuncFlags, FunctionNode},
    import::ImportNode,
    node::Node,
    objective::ObjectiveNode,
    var::VarNode,
    var_info::VarRef,
};

/// The type of an exported object.
/// These will have all their bodies cleared, since it's unnecessary when just being used for type checking.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum ExportType<'a> {
    Constant(ConstantNode<'a>),
    Objective(ObjectiveNode<'a>),
    Function(FunctionNode<'a>),
    Enum(EnumNode<'a>),
    Field(FieldNode<'a>),
    // Blocks aren't exported since they're just private functions called based on Minecraft's "hooks" (tags)
}

pub type TypeRef<'a> = &'a str; // TODO

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Scope<'a> {
    /// The module, function, or block name.
    pub name: &'a str,

    /// A map of local variables to their value types.
    pub locals: BTreeMap<&'a str, Rc<VarNode<'a>>>,

    /// A map of types to user-defined instance methods.
    pub instance_funcs: BTreeMap<TypeRef<'a>, BTreeMap<&'a str, Rc<FunctionNode<'a>>>>,

    /// A map of types to their fields' types.
    pub fields: BTreeMap<TypeRef<'a>, BTreeMap<&'a str, Rc<FieldNode<'a>>>>,

    /// All the symbols the module exports.
    pub exports: BTreeMap<&'a str, Vec<ExportType<'a>>>,

    pub imports: Vec<ImportNode<'a>>,
    pub constants: BTreeMap<&'a str, Rc<ConstantNode<'a>>>,
    pub objectives: BTreeMap<&'a str, Rc<ObjectiveNode<'a>>>,
    pub functions: BTreeMap<&'a str, Rc<FunctionNode<'a>>>,
    pub blocks: Vec<BlockNode<'a>>,
    pub enums: BTreeMap<&'a str, EnumNode<'a>>,

    /// The parent scopes.
    pub parents: Vec<Rc<RefCell<Scope<'a>>>>,
}

impl<'a> Scope<'a> {
    pub fn new(name: &'a str, parents: Vec<Rc<RefCell<Scope<'a>>>>) -> Self {
        Self {
            name,
            parents,
            ..Default::default()
        }
    }

    pub fn lookup(&self, var: &'a str) -> Option<VarRef<'a>> {
        match self.locals.get(var) {
            Some(it) => Some(VarRef::Local(Rc::clone(it))),
            None => self
                .constants
                .get(var)
                .map(|it| VarRef::Const(Rc::clone(it)))
                .or(self
                    .objectives
                    .get(var)
                    .map(|it| VarRef::Objective(Rc::clone(it))))
                .or(self.parents.iter().find_map(|it| it.borrow().lookup(var))),
        }
    }

    pub fn lookup_fn(&self, name: &'a str) -> Option<Rc<FunctionNode<'a>>> {
        match self.functions.get(name) {
            Some(it) => Some(Rc::clone(&it)),
            None => self
                .parents
                .iter()
                .find_map(|it| it.borrow().lookup_fn(name)),
        }
    }

    pub fn lookup_inst_fn(
        &self,
        _ty: &TypeRef<'a>,
        _name: &'a str,
    ) -> Option<Rc<FunctionNode<'a>>> {
        // TODO

        // let res = match self.instance_funcs.get(ty).map(|it| it.get(name)).flatten() {
        //     Some(it) => Some(Rc::clone(&it)),
        //     None => self
        //         .parents
        //         .iter()
        //         .find_map(|it| it.borrow().lookup_inst_fn(ty, name)),
        // };

        // match res {
        //     Some(it) => Some(it),
        //     _ => {
        //         if !ty.is_any() {
        //             self.lookup_inst_fn(&TypeRef::BuiltIn(BuiltInType::Any), name)
        //         } else {
        //             None
        //         }
        //     }
        // }

        None
    }

    pub fn lookup_field(&self, _ty: &TypeRef<'a>, _name: &'a str) -> Option<Rc<FieldNode<'a>>> {
        // TODO

        // let res = match self.fields.get(ty).map(|it| it.get(name)).flatten() {
        //     Some(it) => Some(it),
        //     None => self.parents.iter().find_map(|it| it.lookup_field(ty, name)),
        // };

        // match res {
        //     Some(it) => Some(it),
        //     _ => {
        //         if !ty.is_any() {
        //             self.lookup_field(&TypeRef::BuiltIn(BuiltInType::Any), name)
        //         } else {
        //             None
        //         }
        //     }
        // }

        None
    }

    pub fn add_local(&mut self, name: &'a str, value: VarNode<'a>) {
        debug!("Scope [{}] -> adding local: {name}", self.name);

        self.locals.insert(name, Rc::new(value));
    }
}

pub fn collect_exports<'a>(nodes: &Vec<Node<'a>>) -> BTreeMap<&'a str, Vec<ExportType<'a>>> {
    let mut map = BTreeMap::new();

    for node in nodes {
        match node {
            Node::Constant(var) => {
                if var.is_public {
                    map.entry(var.name.0)
                        .or_insert(Vec::new())
                        .push(ExportType::Constant(var.clone()));
                }
            }

            Node::Objective(obj) => {
                if obj.is_public {
                    map.entry(obj.name.0)
                        .or_insert(Vec::new())
                        .push(ExportType::Objective(obj.clone()));
                }
            }

            Node::Function(func) => {
                if func.flags.contains(FuncFlags::Public) {
                    map.entry(func.name.0)
                        .or_insert(Vec::new())
                        .push(ExportType::Function(func.clone()));
                }
            }

            Node::Enum(en) => map
                .entry(en.name.0)
                .or_insert(Vec::new())
                .push(ExportType::Enum(en.clone())),

            Node::Field(it) => {
                if it.is_public {
                    map.entry(it.name.0)
                        .or_insert(Vec::new())
                        .push(ExportType::Field(it.clone()));
                }
            }

            _ => (),
        };
    }

    map
}
