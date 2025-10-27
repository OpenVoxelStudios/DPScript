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

pub use err::Result;

use crate::dpscript::{
    ast::{
        ast::{AST, ExportType, Scope},
        func::FunctionNode,
        node::Node,
    },
    validator::err::{AllErrors, Err, VErr, Warn},
};
use std::{collections::HashMap, sync::Arc};

pub struct Validator {
    /// The current AST we are validating.
    pub ast: AST,

    /// The resolved imports, for processing the module top-down.
    pub imports: HashMap<String, ExportType>,

    /// A map available modules.
    pub modules: Arc<HashMap<String, AST>>,

    /// The errors generated during validation.
    pub errors: Vec<Err>,

    /// The warnings generated during validation.
    pub warnings: Vec<Warn>,

    /// The scope stack(TM).
    pub scopes: Vec<Scope>,

    /// The function stack(TM).
    pub funcs: Vec<FunctionNode>,

    /// The global scope.
    pub global_scope: Scope,
}

pub struct ValidationResult {
    pub ast: AST,
    pub imports: HashMap<String, ExportType>,
    pub errors: AllErrors,
}

impl Validator {
    pub fn new(ast: AST, modules: Arc<HashMap<String, AST>>) -> Self {
        let mut me = Self {
            global_scope: Scope::new(ast.module.clone(), Vec::new()),
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

    pub fn run(mut self) -> Result<ValidationResult> {
        self.validate_imports()?;
        self.validate_objectives()?;
        self.validate_constants()?;
        self.validate_funcs()?;
        self.validate_blocks()?;

        Ok(ValidationResult {
            errors: AllErrors {
                code: self.ast.code.clone(),
                errors: self.errors,
                warnings: self.warnings,
            },

            ast: self.ast,
            imports: self.imports,
        })
    }

    pub fn scope(&self) -> Result<&Scope> {
        self.scopes.last().ok_or(VErr::NoScope)
    }

    pub fn func(&self) -> Result<&FunctionNode> {
        self.funcs.last().ok_or(VErr::NoFunc)
    }

    pub fn scope_mut(&mut self) -> Result<&mut Scope> {
        self.scopes.last_mut().ok_or(VErr::NoScope)
    }

    pub fn validate(&mut self, node: &mut Node) -> Result<()> {
        match node {
            Node::Import(v) => self.validate_import(v)?,
            Node::Constant(v) => self.validate_constant(v)?,
            Node::Function(v) => self.validate_func(v)?,
            Node::Variable(v) => self.validate_variable(v)?,
            Node::Block(v) => self.validate_block(v)?,
            Node::BinaryOp(v) => self.validate_binop(v)?,
            Node::Enum(v) => self.validate_enum(v)?,
            Node::Conditional(v) => self.validate_cond(v)?,
            Node::UnaryOp(v) => self.validate_unop(v)?,
            Node::Literal(v) => self.validate_literal(v)?,
            Node::Call(v) => self.validate_call(v)?,
            Node::Ident(v) => self.validate_ident_node(v)?,
            Node::Loop(v) => self.validate_loop(v)?,
            Node::Objective(v) => self.validate_objective(v)?,
            Node::Return(v) => self.validate_return(v)?,
            Node::Special(v) => self.validate_special(v)?,
            Node::At(v) => self.validate_at(v)?,
            Node::Field(v) => self.validate_field(v)?,
        }

        Ok(())
    }
}
