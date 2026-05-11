pub mod arr;
pub mod binop;
pub mod literal;
pub mod nbt;
pub mod refs;
pub mod unary;

use crate::prelude::expr::call;

crate::nodes::util::node_group! {
    Value = [
        binop::BinOp,
        literal::Literal,
        literal::DslLiteral,
        call::Call,
        refs::ValueRef,
        refs::VarRef,
        unary::Unary,
        nbt::NbtLiteral,
        arr::ArrayLiteral,
    ]
}
