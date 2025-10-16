use std::fmt;

use dpscript_macros::HasSpan;
use flexstr::SharedStr;
use miette::SourceSpan;

use crate::dpscript::{
    ast::{ast::Scope, node::Node},
    data::NodeInfo,
    ty::TypeRef,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct CallNode {
    pub span: SourceSpan,

    /// A reference to a node that is the receiver,
    /// like when calling an object instance function.
    pub receiver: Vec<SharedStr>,

    /// The name of the function to call.
    pub func: SharedStr,

    /// The arguments the function was called with.
    pub args: Vec<Node>,
}

impl NodeInfo for CallNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }

    fn returns(&self, scope: &Scope) -> Option<TypeRef> {
        match self.receiver.len() {
            1.. => {
                let mut current = None;

                for id in &self.receiver {
                    if let Some(ty) = current {
                        current = Some(scope.fields.get(&ty)?.get(id)?.ty.clone());
                    } else {
                        current = scope.lookup(id)?.compute_ty(scope);
                    }
                }

                scope
                    .lookup_inst_fn(&current?, &self.func)
                    .map(|it| it.return_type.clone())
            }

            0 => scope.lookup_fn(&self.func).map(|it| it.return_type.clone()),
        }
    }
}

impl fmt::Display for CallNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let recv = if !self.receiver.is_empty() {
            format!("[recv: {}] ", self.receiver.join(", "))
        } else {
            "".into()
        };

        write!(
            f,
            "call {}{}: [{}]",
            recv,
            self.func,
            self.args
                .iter()
                .map(|it| format!("{it}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
