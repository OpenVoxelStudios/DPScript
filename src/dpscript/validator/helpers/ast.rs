use super::ModuleExport;
use crate::{AST, CheckerContext, Result};
use std::collections::BTreeMap;

impl AST {
    pub fn cache_values(&mut self) -> Result<()> {
        if self.cached {
            return Ok(());
        }

        self.indexed = false;

        self.index()?;
        self.export_nodes()?; // Cache export nodes

        let cx = self.create_checker_context()?;

        for node in &mut self.nodes {
            node.cache(&cx)?;
        }

        self.indexed = false;

        // Re-run everything with the new set of nodes
        self.index()?;
        self.export_nodes()?;

        let cx = self.create_checker_context()?;

        for node in &mut self.nodes {
            node.cache(&cx)?;
        }

        self.cached = true;

        Ok(())
    }

    pub fn create_checker_context(&mut self) -> Result<CheckerContext> {
        let ast = self.index()?.clone();
        let modules = ast.modules.unwrap();
        let imports = ast.imports.unwrap();
        let funcs = ast.funcs.unwrap();
        let vars = ast.vars.unwrap();
        let blocks = ast.blocks.unwrap();
        let enums = ast.enums.unwrap();
        let objectives = ast.objectives.unwrap();
        let exports = ast.exports.unwrap();

        Ok(CheckerContext {
            modules: modules.clone(),
            imports,
            funcs,
            vars,
            blocks,
            enums,
            objectives,
            exports,

            // State
            cur_fn: None,
            cur_block: None,
            cur_conds: Vec::new(),
            cur_elses: Vec::new(),
            cur_loops: Vec::new(),
            cur_modules: Vec::new(),
            cur_subroutines: Vec::new(),
        })
    }

    pub fn export_nodes(&mut self) -> Result<BTreeMap<String, Vec<ModuleExport>>> {
        if let Some(nodes) = &self.export_nodes {
            return Ok(nodes.clone());
        }

        let cx = self.create_checker_context()?;
        let mut map = BTreeMap::new();

        if let Some(modules) = &self.modules {
            for (name, module) in modules {
                map.insert(name.clone(), module.get_exports(&cx)?);
            }
        }

        self.export_nodes = Some(map.clone());

        Ok(map)
    }
}
