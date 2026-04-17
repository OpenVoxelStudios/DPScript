use crate::{constant::ConstantNode, node::Node, objective::ObjectiveNode, var::VarNode};
use std::rc::Rc;

pub enum VarRef<'a> {
    Local(Rc<VarNode<'a>>),
    Const(Rc<ConstantNode<'a>>),
    Objective(Rc<ObjectiveNode<'a>>),
}

impl<'a> VarRef<'a> {
    pub fn as_node(&self) -> Node<'a> {
        match self {
            VarRef::Local(it) => Node::Variable(Rc::unwrap_or_clone(Rc::clone(it))),
            VarRef::Const(it) => Node::Constant(Rc::unwrap_or_clone(Rc::clone(it))),
            VarRef::Objective(it) => Node::Objective(Rc::unwrap_or_clone(Rc::clone(it))),
        }
    }
}
