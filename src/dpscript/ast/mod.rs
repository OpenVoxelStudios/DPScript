mod node;

pub use node::*;

use crate::{ModuleExport, Result, module_indexer_add};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct AST {
    pub nodes: Vec<Node>,
    pub indexed: bool,
    pub cached: bool,

    // We want these sorted for niceties, so we use a BTreeMap
    #[serde(skip)]
    pub modules: Option<BTreeMap<String, Module>>,

    #[serde(skip)]
    pub top_level: Option<BTreeMap<String, Vec<TopLevelNode>>>,

    #[serde(skip)]
    pub imports: Option<BTreeMap<String, Vec<Import>>>,

    #[serde(skip)]
    pub funcs: Option<BTreeMap<String, Vec<Function>>>,

    #[serde(skip)]
    pub vars: Option<BTreeMap<String, Vec<Variable>>>,

    #[serde(skip)]
    pub blocks: Option<BTreeMap<String, Vec<Block>>>,

    #[serde(skip)]
    pub enums: Option<BTreeMap<String, Vec<Enum>>>,

    #[serde(skip)]
    pub objectives: Option<BTreeMap<String, Vec<Objective>>>,

    #[serde(skip)]
    pub exports: Option<BTreeMap<String, Vec<Export>>>,

    #[serde(skip)]
    pub export_nodes: Option<BTreeMap<String, Vec<ModuleExport>>>, // just a cache :) this is updated elsewhere
}

impl AST {
    pub fn merge(&mut self, other: AST) -> &mut Self {
        self.nodes.extend(other.nodes);
        self
    }

    pub fn collect_modules(&self) -> BTreeMap<String, Module> {
        let mut modules: BTreeMap<String, Module> = BTreeMap::new();

        for node in &self.nodes {
            if let Node::Module(module) = node {
                let name = module.name();

                if let Some(it) = modules.get_mut(&name) {
                    it.body.extend(module.no_submodules());
                } else {
                    modules.insert(name.clone(), module.with_no_submodules());
                }

                for (name, item) in module.collect_submodules(name) {
                    if let Some(it) = modules.get_mut(&name) {
                        it.body.extend(item.body);
                    } else {
                        modules.insert(name, item);
                    }
                }
            }
        }

        modules
    }

    pub fn index(&mut self) -> Result<&mut Self> {
        if self.indexed {
            return Ok(self);
        }

        let mut modules = self.collect_modules();

        for (_, module) in &mut modules {
            module.index_top_level_nodes()?;
        }

        self.modules = Some(modules.clone());

        let mut top_level: BTreeMap<String, Vec<TopLevelNode>> = BTreeMap::new();
        let mut imports: BTreeMap<String, Vec<Import>> = BTreeMap::new();
        let mut funcs: BTreeMap<String, Vec<Function>> = BTreeMap::new();
        let mut vars: BTreeMap<String, Vec<Variable>> = BTreeMap::new();
        let mut blocks: BTreeMap<String, Vec<Block>> = BTreeMap::new();
        let mut enums: BTreeMap<String, Vec<Enum>> = BTreeMap::new();
        let mut objectives: BTreeMap<String, Vec<Objective>> = BTreeMap::new();
        let mut exports: BTreeMap<String, Vec<Export>> = BTreeMap::new();

        for (name, module) in modules {
            module_indexer_add!(top_level += (name, module));
            module_indexer_add!(imports += (name, module));
            module_indexer_add!(funcs += (name, module));
            module_indexer_add!(vars += (name, module));
            module_indexer_add!(blocks += (name, module));
            module_indexer_add!(enums += (name, module));
            module_indexer_add!(objectives += (name, module));
            module_indexer_add!(exports += (name, module));
        }

        self.top_level = Some(top_level);
        self.imports = Some(imports);
        self.funcs = Some(funcs);
        self.vars = Some(vars);
        self.blocks = Some(blocks);
        self.enums = Some(enums);
        self.objectives = Some(objectives);
        self.exports = Some(exports);
        self.indexed = true;

        Ok(self)
    }
}
