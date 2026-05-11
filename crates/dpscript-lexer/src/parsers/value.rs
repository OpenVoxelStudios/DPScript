use crate::{
    Result,
    cx::ParseCx,
    err::Error,
    parsers::{
        binop::{parse_array_index, parse_binop},
        call::parse_call,
        literal::{parse_array_literal, parse_dsl_literal, parse_literal, parse_nbt_literal},
        refs::{parse_value_ref, parse_var_ref},
        unary::parse_unary,
    },
    util::TokenCursor,
};
use dpscript_ast::prelude::value::Value;

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_value<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Value<'a>> {
    c.save();

    match parse_unary(c, cx) {
        Ok(it) => return Ok(Value::Unary(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_binop(c, cx) {
        Ok(it) => return Ok(Value::BinOp(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_array_index(c, cx) {
        Ok(it) => return Ok(Value::BinOp(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_call(c, cx) {
        Ok(it) => return Ok(Value::Call(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_value_ref(c, cx) {
        Ok(it) => return Ok(Value::ValueRef(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_var_ref(c, cx) {
        Ok(it) => return Ok(Value::VarRef(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_literal(c, cx) {
        Ok(it) => return Ok(Value::Literal(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_dsl_literal(c, cx) {
        Ok(it) => return Ok(Value::DslLiteral(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_nbt_literal(c, cx) {
        Ok(it) => return Ok(Value::NbtLiteral(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_array_literal(c, cx) {
        Ok(it) => return Ok(Value::ArrayLiteral(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    Err(cx.unexpected(c.take_next()?))
}
