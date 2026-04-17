pub mod at;
pub mod binop;
pub mod blocks;
pub mod call;
pub mod cond;
pub mod consts;
pub mod enums;
pub mod err;
pub mod field;
pub mod funcs;
pub mod idents;
pub mod import;
pub mod literal;
pub mod loops;
pub mod objective;
pub mod ret;
pub mod special;
pub mod unop;
pub mod vars;

use ast::{
    ast::AST,
    func::FunctionNode,
    node::Node,
    scope::{ExportType, Scope},
};
pub use err::Result;

use crate::dpscript::validator::err::{AllErrors, Err, VErr, Warn};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

pub struct Validator<'a> {
    /// The current AST we are validating.
    pub ast: Rc<RefCell<AST<'a>>>,

    /// The resolved imports, for processing the module top-down.
    pub imports: HashMap<&'a str, ExportType<'a>>,

    /// A map available modules.
    pub modules: Arc<HashMap<&'a str, Rc<RefCell<AST<'a>>>>>,

    /// The errors generated during validation.
    pub errors: Vec<Err>,

    /// The warnings generated during validation.
    pub warnings: Vec<Warn>,

    /// The scope stack(TM).
    pub scopes: Vec<Rc<RefCell<Scope<'a>>>>,

    /// The function stack(TM).
    pub funcs: Vec<FunctionNode<'a>>,

    /// The global scope.
    pub global_scope: Rc<RefCell<Scope<'a>>>,
}

pub struct ValidationResult<'a> {
    pub ast: Rc<RefCell<AST<'a>>>,
    pub imports: HashMap<&'a str, ExportType<'a>>,
    pub errors: AllErrors,
}

impl<'a> Validator<'a> {
    pub fn new(
        ast: Rc<RefCell<AST<'a>>>,
        modules: Arc<HashMap<&'a str, Rc<RefCell<AST<'a>>>>>,
    ) -> Self {
        let mut me = Self {
            global_scope: Rc::new(RefCell::new(Scope::new(
                Rc::clone(&ast).borrow().module.clone(),
                Vec::new(),
            ))),

            ast,
            modules,
            imports: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            scopes: Vec::new(),
            funcs: Vec::new(),
        };

        me.scopes.push(me.global_scope.clone());
        me
    }

    pub fn run(mut self) -> Result<ValidationResult<'a>> {
        self.validate_imports()?;
        self.validate_objectives()?;
        self.validate_constants()?;
        self.validate_funcs()?;
        self.validate_blocks()?;

        let code = self.ast.borrow().code.clone().into();

        Ok(ValidationResult {
            errors: AllErrors {
                code,
                errors: self.errors,
                warnings: self.warnings,
            },

            ast: self.ast,
            imports: self.imports,
        })
    }

    pub fn scope(&self) -> Result<Rc<RefCell<Scope<'a>>>> {
        self.scopes.last().cloned().ok_or(VErr::NoScope)
    }

    pub fn func(&self) -> Result<&FunctionNode<'a>> {
        self.funcs.last().ok_or(VErr::NoFunc)
    }

    pub fn validate(&mut self, node: &mut Node<'a>) -> Result<()> {
        match node {
            Node::Import(v) => self.validate_import(v)?,
            Node::Variable(v) => self.validate_variable(v)?,
            Node::Block(v) => self.validate_block(v)?,
            Node::BinaryOp(v) => self.validate_binop(v)?,
            Node::Enum(v) => self.validate_enum(v)?,
            Node::Conditional(v) => self.validate_cond(v)?,
            Node::UnaryOp(v) => self.validate_unop(v)?,
            Node::Literal(v) => self.validate_literal(v)?,
            Node::Call(v) => self.validate_call(v)?,
            Node::Loop(v) => self.validate_loop(v)?,
            Node::Objective(v) => self.validate_objective(v)?,
            Node::Return(v) => self.validate_return(v)?,
            Node::Special(v) => self.validate_special(v)?,
            Node::At(v) => self.validate_at(v)?,
            Node::Field(v) => self.validate_field(v)?,
            Node::Ref(_ref_node) => todo!(),

            Node::Function(v) => {
                let copy = self.validate_func(v.clone())?;

                *v = Rc::into_inner(copy).expect("Failed to unwrap Rc on function node!");
            }

            Node::Constant(v) => {
                let copy = self.validate_constant(v.clone())?;

                *v = Rc::into_inner(copy).expect("Failed to unwrap Rc on constant node!");
            }
        }

        Ok(())
    }
}
