use std::fmt;

use dpscript_macros::HasSpan;
use flexstr::SharedStr;
use miette::SourceSpan;

use crate::dpscript::{ast::ast::Scope, data::NodeInfo, ty::TypeRef};

/// A node referencing the name of another node in the AST.
/// This is typically used to reference variables.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct IdentNode {
    pub span: SourceSpan,
    pub ident: SharedStr,
}

impl NodeInfo for IdentNode {
    fn is_const(&self, scope: &Scope) -> bool {
        scope
            .lookup(&self.ident)
            .map(|it| it.is_const_var())
            .unwrap_or(false)
    }

    fn returns(&self, scope: &Scope) -> Option<TypeRef> {
        scope
            .lookup(&self.ident)
            .map(|it| it.compute_ty(scope))
            .flatten()
    }
}

impl fmt::Display for IdentNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ident)
    }
}
