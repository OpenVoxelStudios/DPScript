use super::{ImportNode, Node, TopLevelNode};
use crate::{ModuleImport, Result, Spanned, ValidatorError, module_top_level_getter};
use miette::{NamedSource, SourceSpan};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct Module {
    /// Is the module public? If it's `module whatever;` at the top of the file then it always is.
    pub is_pub: bool,

    /// A list of parts of this module's name.
    pub name: Vec<Spanned<String>>,

    /// The span.
    pub span: SourceSpan,

    /// The body.
    pub body: Vec<Node>,

    /// The top level nodes. This is a cache. If this is none, then we are still in the lexer.
    pub top_level: Option<Vec<TopLevelNode>>,

    /// The file & source this module is in.
    #[serde(skip)]
    pub source: NamedSource<String>,

    /// A cache for imported objects.
    #[serde(skip)]
    pub imported_objects: Option<Vec<ModuleImport>>,
}

impl Module {
    pub fn source(&self) -> NamedSource<String> {
        self.source.clone()
    }

    pub fn no_submodules(&self) -> Vec<Node> {
        let mut nodes = Vec::new();

        for node in &self.body {
            match node {
                Node::Module(_) => {}
                other => nodes.push(other.clone()),
            };
        }

        nodes
    }

    pub fn with_no_submodules(&self) -> Self {
        let mut body = Vec::new();

        for node in &self.body {
            match node {
                Node::Module(_) => {}
                other => body.push(other.clone()),
            };
        }

        Self {
            body,
            is_pub: self.is_pub,
            name: self.name.clone(),
            span: self.span,
            top_level: self.top_level.clone(),
            source: self.source.clone(),
            imported_objects: None,
        }
    }

    pub fn collect_submodules(&self, base: String) -> HashMap<String, Module> {
        let mut modules: HashMap<String, Module> = HashMap::new();

        for node in &self.body {
            if let Node::Module(module) = node {
                let name = format!(
                    "{}/{}",
                    base,
                    module
                        .name
                        .iter()
                        .map(|v| v.0.clone())
                        .collect::<Vec<_>>()
                        .join("/")
                );

                if let Some(it) = modules.get_mut(&name) {
                    it.body.extend(module.body.clone());
                } else {
                    modules.insert(name.clone(), module.clone());
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

    pub fn top_level_nodes(&self) -> Result<Vec<TopLevelNode>> {
        let mut nodes = Vec::new();

        for node in &self.body {
            nodes.push(match node.clone() {
                Node::Module(m) => TopLevelNode::Module(m),
                Node::Import(i) => TopLevelNode::Import(i),
                Node::Function(f) => TopLevelNode::Function(f),
                Node::Variable(v) => TopLevelNode::Variable(v),
                Node::Block(b) => TopLevelNode::Block(b),
                Node::Enum(e) => TopLevelNode::Enum(e),
                Node::Objective(o) => TopLevelNode::Objective(o),
                Node::Export(e) => TopLevelNode::Export(e),

                _ => {
                    return Err(ValidatorError {
                        src: self.source.clone(),
                        at: node.span(),
                        err: format!("This node is not allowed in the top-level: {:?}", node),
                    }
                    .into());
                }
            });
        }

        Ok(nodes)
    }

    pub fn top_level(&self) -> Vec<TopLevelNode> {
        self.top_level.clone().unwrap_or_default()
    }

    pub fn index_top_level_nodes(&mut self) -> Result<()> {
        self.top_level = Some(self.top_level_nodes()?);

        Ok(())
    }

    pub fn get_imported_names(&self) -> Vec<String> {
        let mut imports = Vec::new();

        for item in &self.body {
            match item {
                Node::Import(it) => {
                    for node in &it.imports {
                        match node {
                            ImportNode::Object((obj, _)) => imports.push(obj.clone()),
                            _ => todo!("Implement nested imports"),
                        }
                    }
                }

                _ => {}
            }
        }

        imports
    }

    pub fn get_enums(&self) -> Vec<(String, Vec<String>)> {
        let mut enums = Vec::new();

        for item in &self.body {
            match item {
                Node::Enum(it) => {
                    let mut entries = Vec::new();

                    for (entry, _) in &it.entries {
                        entries.push(entry.clone());
                    }

                    enums.push((it.name.0.clone(), entries));
                }

                _ => {}
            }
        }

        enums
    }

    pub fn name(&self) -> String {
        self.name
            .iter()
            .map(|v| v.0.clone())
            .collect::<Vec<_>>()
            .join("/")
    }
}

module_top_level_getter!(imports -> Import);
module_top_level_getter!(funcs -> Function);
module_top_level_getter!(vars -> Variable);
module_top_level_getter!(blocks -> Block);
module_top_level_getter!(enums -> Enum);
module_top_level_getter!(objectives -> Objective);
module_top_level_getter!(exports -> Export);
module_top_level_getter!(modules -> Module);
