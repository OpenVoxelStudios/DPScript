pub mod assign;
pub mod block;
pub mod call;
pub mod ret;
pub mod var;

use crate::prelude::def::constant;
use block::*;

crate::nodes::util::node_group! {
    Expr = [
        @call::Call,
        @at::At,
        @cond::Cond,
        @loops::ForLoop,
        @ret::Return,
        @var::Variable,
        @constant::Constant,
        @assign::Assign,
    ]
}
