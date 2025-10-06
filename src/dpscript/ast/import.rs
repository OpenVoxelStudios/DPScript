use std::fmt;

use miette::SourceSpan;

use crate::dpscript::{
    ast::{ast::Scope, ident::IdentNode},
    data::NodeInfo,
    ty::TypeRef,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ImportNode {
    pub span: SourceSpan,

    /// A list of paths to imports.
    pub imports: Vec<Vec<IdentNode>>,
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
            for import in set {
                write!(f, "import [{import}];\n")?;
            }
        }

        Ok(())
    }
}
