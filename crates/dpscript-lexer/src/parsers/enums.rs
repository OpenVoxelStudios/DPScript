use crate::{
    Result,
    cx::ParseCx,
    parsers::meta::{parse_def_flags, parse_def_meta},
    util::TokenCursor,
};
use dpscript_ast::prelude::def::enums::{Enum, EnumValue, EnumVariant};
use dpscript_parser::{Assignment, BraceType, Keyword, Literal, Punct, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
fn parse_enum_variant<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<EnumVariant<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let name = c.expect_ident()?;
    let mut value = EnumValue::None;

    if c.next_if_eq(&Token::Assignment(Assignment::Equal))
        .is_some()
    {
        match c.take_next()? {
            (Token::Literal(Literal::String(it)), span) => {
                value = EnumValue::String((it, span));
            }

            (Token::Literal(Literal::Byte(it)), span) => {
                value = EnumValue::Byte((it, span));
            }

            (Token::Literal(Literal::Int(it)), span) => {
                // TODO: Throw an error if it's out of bounds
                value = EnumValue::Byte((it as i8, span));
            }

            other => return Err(cx.unexpected(other)),
        }
    }

    // TODO: Make this optional if it's not the last? Idk tho, it's not a big deal....
    c.expect(Token::Punct(Punct::Comma))?;

    let span = c.end_span();

    Ok(EnumVariant {
        name,
        span,
        value,
        meta,
    })
}

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_enum<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Enum<'a>> {
    c.begin_span();

    let flags = parse_def_flags(c, cx)?;

    c.expect_or_skip(Token::Keyword(Keyword::Enum))?;

    let name = c.expect_ident()?;
    let mut inner = c.expect_group(BraceType::Braces)?;
    let mut variants = Vec::new();

    while inner.has_next() {
        variants.push(parse_enum_variant(&mut inner, cx)?);
    }

    inner.assert_empty()?;

    let span = c.end_span();

    Ok(Enum {
        name,
        flags,
        variants,
        span,
        meta: Default::default(),
    })
}
