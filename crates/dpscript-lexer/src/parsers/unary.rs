use crate::{Result, cx::ParseCx, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::{
    HasSpan,
    value::unary::{Unary, UnaryOp},
};
use dpscript_parser::{Operator, Token};

pub fn parse_unary<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<'a, Unary<'a>> {
    if let Some((_, span)) = c.next_if_eq(&Token::Operator(Operator::Minus)) {
        let value = parse_value(c, cx)?;
        let span = span + value.span();

        Ok(Unary {
            op: UnaryOp::Negate,
            span,
            value: Box::new(value),
        })
    } else if let Some((_, span)) = c.next_if_eq(&Token::Operator(Operator::Bang)) {
        let value = parse_value(c, cx)?;
        let span = span + value.span();

        Ok(Unary {
            op: UnaryOp::Invert,
            span,
            value: Box::new(value),
        })
    } else {
        Err(cx.unexpected(c.take_next()?))
    }
}
