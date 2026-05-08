pub mod binop;
pub mod literal;
pub mod refs;
pub mod unary;

use crate::prelude::expr::call;

crate::nodes::util::node_group! {
    Value = [
        binop::BinOp,
        literal::Literal,
        call::Call,
        refs::ValueRef,
        refs::VarRef,
        unary::Unary,
    ]
}
