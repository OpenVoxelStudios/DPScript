use crate::dpscript::{ast::ast::Scope, data::NodeInfo, ty::TypeRef};
use miette::SourceSpan;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ImportNode {
    pub span: SourceSpan,

    /// A list of paths to imports (each part separated by '::' is a different element in the sub-Vec).
    pub imports: Vec<Vec<String>>,
}

impl NodeInfo for ImportNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        true
    }

    fn returns(&self, _scope: &Scope) -> Option<TypeRef> {
        None
    }
}

impl fmt::Display for ImportNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for set in &self.imports {
            write!(f, "import [{}];", set.join(", "))?;
        }

        Ok(())
    }
}
