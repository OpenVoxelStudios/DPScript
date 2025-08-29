use crate::dpscript::{ast::ast::Scope, ty::TypeRef};

pub trait NodeInfo {
    fn is_const(&self, scope: &Scope) -> bool;

    /// Get the type of data the node will return.
    /// This wil be [`None`] if it has no value.
    fn returns(&self, _scope: &Scope) -> Option<TypeRef> {
        None
    }
}

impl<T: NodeInfo> NodeInfo for Vec<T> {
    fn is_const(&self, scope: &Scope) -> bool {
        self.iter().all(|it| it.is_const(scope))
    }
}
