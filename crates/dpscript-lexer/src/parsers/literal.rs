use crate::{Result, cx::ParseCx, util::TokenCursor};
use dpscript_ast::prelude::value::literal::{Literal, LiteralValue};
use dpscript_parser::{Keyword, Literal as TLiteral, Token};

pub fn parse_literal<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<'a, Literal<'a>> {
    match c.next() {
        Some((Token::Literal(TLiteral::Byte(it)), span)) => Ok(Literal {
            value: LiteralValue::Byte(it),
            span,
        }),

        Some((Token::Literal(TLiteral::Double(it)), span)) => Ok(Literal {
            value: LiteralValue::Double(it),
            span,
        }),

        Some((Token::Literal(TLiteral::Float(it)), span)) => Ok(Literal {
            value: LiteralValue::Float(it),
            span,
        }),

        Some((Token::Literal(TLiteral::Int(it)), span)) => Ok(Literal {
            value: LiteralValue::Int(it),
            span,
        }),

        Some((Token::Literal(TLiteral::Long(it)), span)) => Ok(Literal {
            value: LiteralValue::Long(it),
            span,
        }),

        Some((Token::Literal(TLiteral::String(it)), span)) => Ok(Literal {
            value: LiteralValue::String(it),
            span,
        }),

        Some((Token::Keyword(Keyword::True), span)) => Ok(Literal {
            value: LiteralValue::Bool(true),
            span,
        }),

        Some((Token::Keyword(Keyword::False), span)) => Ok(Literal {
            value: LiteralValue::Bool(false),
            span,
        }),

        Some(tkn) => Err(cx.unexpected(tkn)),
        None => Err(cx.eof(c.cur_span())),
    }
}
