pub mod arr;
pub mod binop;
pub mod literal;
pub mod nbt;
pub mod refs;
pub mod unary;

use dpscript_core::SourceSpan;

use crate::prelude::{expr::call, types::TypeRef};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct TypedValue<'a> {
    pub value: Box<Value<'a>>,
    pub ty: TypeRef<'a>,
    pub span: SourceSpan,
}

crate::nodes::util::node_group! {
    Value = [
        @binop::BinOp,
        @literal::Literal,
        @literal::DslLiteral,
        @call::Call,
        @refs::ValueRef,
        @refs::VarRef,
        @unary::Unary,
        @nbt::NbtLiteral,
        @arr::ArrayLiteral,
        TypedValue,
    ]
}

impl<'a> Default for Value<'a> {
    fn default() -> Self {
        Self::Literal(literal::Literal::default())
    }
}
