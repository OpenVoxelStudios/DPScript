use dpscript_macros::HasSpan;
use miette::SourceSpan;

use crate::dpscript::{ast::{ast::Scope, node::Node}, data::NodeInfo, ty::TypeRef};

/// A node referencing the name of another node in the AST.
/// This is typically used to reference variables.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct IdentNode {
    pub span: SourceSpan,
    pub ident: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct FieldNode {
    pub span: SourceSpan,
    pub receiver: Box<Node>,
    pub field: String,
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

impl NodeInfo for FieldNode {
    fn is_const(&self, scope: &Scope) -> bool {
        self.receiver.is_const(scope)
    }

    // TODO
    fn returns(&self, _scope: &Scope) -> Option<TypeRef> {
        None
    }
}
