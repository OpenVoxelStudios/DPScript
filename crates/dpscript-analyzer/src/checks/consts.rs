use dpscript_ast::nodes::{
    meta::DefFlags,
    value::{
        Value,
        literal::{Literal, LiteralValue},
    },
};

use crate::{cx::VisitCx, scope::ScopeLookupTrait};

/// Check if the current value is constant, in that it will never change and can be inlined.
pub trait ConstCheckable<'a, 'visit> {
    fn is_const(&self, cx: &VisitCx<'a, 'visit>) -> bool;
}

impl<'a, 'visit> ConstCheckable<'a, 'visit> for Literal<'a> {
    fn is_const(&self, _cx: &VisitCx<'a, 'visit>) -> bool {
        match self.value {
            LiteralValue::String(_)
            | LiteralValue::Bool(_)
            | LiteralValue::Byte(_)
            | LiteralValue::Int(_)
            | LiteralValue::Long(_)
            | LiteralValue::Float(_)
            | LiteralValue::Double(_)
            | LiteralValue::Null => true,

            LiteralValue::CurPos => false,
        }
    }
}

impl<'a, 'visit> ConstCheckable<'a, 'visit> for Value<'a> {
    fn is_const(&self, cx: &VisitCx<'a, 'visit>) -> bool {
        match self {
            Value::Unary(it) => {
                it.value.is_const(cx)
                    && it
                        .resolved
                        .as_ref()
                        .is_some_and(|it| it.data.flags.contains(&DefFlags::Const))
            }

            Value::BinOp(it) => {
                it.lhs.is_const(cx)
                    && it.rhs.is_const(cx)
                    && it
                        .resolved
                        .as_ref()
                        .is_some_and(|it| it.data.flags.contains(&DefFlags::Const))
            }

            Value::Call(it) => {
                it.target.as_ref().is_none_or(|it| it.is_const(cx))
                    && it.args.iter().all(|it| it.is_const(cx))
                    && it
                        .resolved
                        .as_ref()
                        .is_some_and(|it| it.data.flags.contains(&DefFlags::Const))
            }

            Value::Literal(it) => it.is_const(cx),

            Value::DslLiteral(it) => {
                it.value.is_const(cx)
                    && it
                        .resolved
                        .as_ref()
                        .is_some_and(|it| it.data.flags.contains(&DefFlags::Const))
            }

            Value::ValueRef(_) => false,

            Value::VarRef(it) => cx
                .lookup()
                .lookup_var_or_const(it.name)
                .is_ok_and(|it| it.is_right()),

            Value::NbtLiteral(it) => it.values.iter().all(|(_, it)| it.is_const(cx)),
            Value::ArrayLiteral(it) => it.values.iter().all(|it| it.is_const(cx)),
            Value::TypedValue(it) => it.value.is_const(cx),
        }
    }
}
