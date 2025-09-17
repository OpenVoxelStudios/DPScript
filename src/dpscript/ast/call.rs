use dpscript_macros::HasSpan;
use miette::SourceSpan;

use crate::dpscript::{
    ast::{ast::Scope, node::Node},
    data::NodeInfo,
    ty::TypeRef,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct CallNode {
    pub span: SourceSpan,

    /// A reference to a node that is the receiver,
    /// like when calling an object instance function.
    pub receiver: Option<Box<Node>>,

    /// The name of the function to call.
    pub func: String,

    /// The arguments the function was called with.
    pub args: Vec<Node>,
}

impl NodeInfo for CallNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        // TODO: Constant functions, maybe?
        false
    }

    fn returns(&self, scope: &Scope) -> Option<TypeRef> {
        match &self.receiver {
            Some(recv) => match recv.returns(scope) {
                Some(ret) => scope
                    .module
                    .instance_funcs
                    .get(&ret)
                    .map(|it| {
                        it.get(&self.func)
                            .map(|it| it.return_type.clone())
                            .flatten()
                    })
                    .flatten(),

                None => None,
            },

            None => scope
                .module
                .funcs
                .get(&self.func)
                .map(|it| it.return_type.clone())
                .flatten(),
        }
    }
}
