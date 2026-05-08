use crate::{
    Result,
    cx::ParseCx,
    parsers::meta::{parse_def_flags, parse_def_meta},
    util::TokenCursor,
};
use dpscript_ast::prelude::def::enums::{Enum, EnumValue, EnumVariant};
use dpscript_parser::{Assignment, BraceType, Keyword, Literal, Punct, Token};

fn parse_enum_variant<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<'a, EnumVariant<'a>> {
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

pub fn parse_enum<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<'a, Enum<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let flags = parse_def_flags(c, cx)?;

    c.expect(Token::Keyword(Keyword::Enum))?;

    let name = c.expect_ident()?;
    let mut inner = c.expect_group(BraceType::Braces)?;
    let mut variants = Vec::new();

    while inner.has_next() {
        variants.push(parse_enum_variant(&mut inner, cx)?);
    }

    let span = c.end_span();

    Ok(Enum {
        name,
        flags,
        variants,
        span,
        meta,
    })
}
