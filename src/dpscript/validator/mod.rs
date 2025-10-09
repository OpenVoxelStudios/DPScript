pub mod binop;
pub mod consts;
pub mod err;
pub mod funcs;
pub mod import;

pub use err::Result;

use crate::dpscript::{
    ast::{
        ast::{AST, ExportType, Scope},
        node::Node,
    },
    validator::err::{AllErrors, Err, VErr, Warn},
};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

pub struct Validator {
    /// The current AST we are validating.
    pub ast: Arc<AST>,

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
        let ast = Arc::new(ast);

        Self {
            global_scope: Scope {
                locals: BTreeMap::new(),
                module: Arc::clone(&ast),
            },
            ast,
            modules,
            imports: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            scopes: Vec::new(),
        }
    }

    pub fn run(mut self) -> Result<AllErrors> {
        self.validate_imports()?;
        self.validate_constants()?;
        self.validate_funcs()?;

        Ok(AllErrors {
            code: self.ast.code.clone(),
            errors: self.errors,
            warnings: self.warnings,
        })
    }

    pub fn scope(&self) -> Result<&Scope> {
        self.scopes.last().ok_or(VErr::NoScope)
    }

    pub fn validate(&mut self, _node: &Node) -> Result<()> {
        // TODO: Everything

        Ok(())
    }
}
