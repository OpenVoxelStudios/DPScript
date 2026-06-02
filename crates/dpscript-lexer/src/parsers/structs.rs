use crate::{
    Result,
    cx::ParseCx,
    parsers::{
        meta::{parse_def_flags, parse_def_meta},
        types::parse_typeref,
    },
    util::TokenCursor,
};
use dpscript_ast::prelude::def::structs::{Struct, StructField};
use dpscript_parser::{BraceType, Keyword, Punct, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
fn parse_struct_field<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<StructField<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let name = c.expect_ident()?;

    c.expect(Token::Punct(Punct::Colon))?;

    let ty = parse_typeref(c, cx)?;

    // TODO: Make this optional if it's not the last? Idk tho, it's not a big deal....
    c.expect(Token::Punct(Punct::Comma))?;

    let span = c.end_span();

    Ok(StructField {
        name,
        ty,
        span,
        meta,
    })
}

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_struct<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Struct<'a>> {
    c.begin_span();

    let flags = parse_def_flags(c, cx)?;

    c.expect_or_skip(Token::Keyword(Keyword::Struct))?;

    let name = c.expect_ident()?;
    let mut extends = Vec::new();

    if c.next_if_eq(&Token::Keyword(Keyword::Extends)).is_some() {
        extends.push(parse_typeref(c, cx)?);
    }

    let mut inner = c.expect_group(BraceType::Braces)?;
    let mut fields = Vec::new();

    while inner.has_next() {
        fields.push(parse_struct_field(&mut inner, cx)?);
    }

    inner.assert_empty()?;

    let span = c.end_span();

    Ok(Struct {
        name,
        extends,
        flags,
        fields,
        span,
        meta: Default::default(),
    })
}
