pub mod binop;
pub mod blocks;
pub mod consts;
pub mod enums;
pub mod err;
pub mod funcs;
pub mod idents;
pub mod import;
pub mod unop;
pub mod vars;

pub use err::Result;

use crate::dpscript::{
    ast::{
        ast::{AST, ExportType, Scope},
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

    /// The global scope.
    pub global_scope: Scope,
}

impl Validator {
    pub fn new(ast: AST, modules: Arc<HashMap<String, AST>>) -> Self {
        let mut me = Self {
            global_scope: Scope::default(),
            ast,
            modules,
            imports: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            scopes: Vec::new(),
        };

        me.scopes.push(me.global_scope.clone());
        me
    }

    pub fn run(mut self) -> Result<AllErrors> {
        self.validate_imports()?;
        self.validate_constants()?;
        self.validate_funcs()?;
        self.validate_blocks()?;

        Ok(AllErrors {
            code: self.ast.code.clone(),
            errors: self.errors,
            warnings: self.warnings,
        })
    }

    pub fn scope(&self) -> Result<&Scope> {
        self.scopes.last().ok_or(VErr::NoScope)
    }

    pub fn scope_mut(&mut self) -> Result<&mut Scope> {
        self.scopes.last_mut().ok_or(VErr::NoScope)
    }

    pub fn validate(&mut self, node: &mut Node) -> Result<()> {
        // TODO: Everything

        match node {
            Node::Constant(v) => self.validate_constant(v)?,
            Node::Function(v) => self.validate_func(v)?,
            Node::Variable(v) => self.validate_variable(v)?,
            Node::Block(v) => self.validate_block(v)?,
            _ => {} // TODO
        }

        Ok(())
    }
}
