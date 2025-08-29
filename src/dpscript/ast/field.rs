// use miette::SourceSpan;

// use crate::dpscript::{ast::{ast::Scope, node::Node}, data::NodeInfo, ty::TypeRef};

// /// A node for accessing a field of another object (or field).
// #[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
// pub struct FieldNode {
//     pub span: SourceSpan,
//     pub receiver: Box<Node>,
//     pub field: String,
// }

// impl NodeInfo for FieldNode {
//     fn is_const(&self, _scope: &Scope) -> bool {
//         // TODO: Accessing fields from const contexts?
//         false
//     }

//     fn returns(&self, scope: &Scope) -> Option<TypeRef> {
//         match self.receiver.returns(scope) {
//             Some(it) => scope.module.instance_funcs.get(&it).map(|it| it.get(k))
//         }
//     }
// }
