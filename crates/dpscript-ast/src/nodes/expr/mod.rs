pub mod block;
pub mod call;
pub mod ret;
pub mod unop;
pub mod var;

crate::nodes::util::node_group! { Expr = [call::Call, block::Block, ret::Return, var::Variable] }
