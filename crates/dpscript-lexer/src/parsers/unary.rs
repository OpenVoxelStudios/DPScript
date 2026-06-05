use crate::{Result, cx::ParseCx, err::Error, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::{
    HasSpan,
    value::unary::{Unary, UnaryOp},
};
use dpscript_parser::{Operator, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_unary<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Unary<'a>> {
    if let Some((_, span)) = c.next_if_eq(&Token::Operator(Operator::Minus)) {
        let value = parse_value(c, cx)?;
        let span = span + value.span();

        Ok(Unary {
            op: UnaryOp::Negate,
            span,
            value: Box::new(value),
            resolved: None,
        })
    } else if let Some((_, span)) = c.next_if_eq(&Token::Operator(Operator::Bang)) {
        let value = parse_value(c, cx)?;
        let span = span + value.span();

        Ok(Unary {
            op: UnaryOp::Invert,
            span,
            value: Box::new(value),
            resolved: None,
        })
    } else if let Some((_, span)) = c.next_if_eq(&Token::Operator(Operator::CurPos))
        && c.has_next()
    {
        let value = parse_value(c, cx).map_err(|_| Error::Skip)?;
        let span = span + value.span();

        Ok(Unary {
            op: UnaryOp::Offset,
            span,
            value: Box::new(value),
            resolved: None,
        })
    } else {
        Err(cx.skip())
    }
}
