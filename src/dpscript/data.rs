use ast::scope::Scope;
use crate::dpscript::ty::TypeRef;

pub trait NodeInfo<'a> {
    fn is_const(&self, scope: &Scope<'a>) -> bool;

    /// Get the type of data the node will return.
    /// This wil be [`None`] if it has no value.
    fn returns(&self, _scope: &Scope<'a>) -> Option<TypeRef> {
        None
    }
}

impl<'a, T: NodeInfo<'a>> NodeInfo<'a> for Vec<T> {
    fn is_const(&self, scope: &Scope<'a>) -> bool {
        self.iter().all(|it| it.is_const(scope))
    }
}
