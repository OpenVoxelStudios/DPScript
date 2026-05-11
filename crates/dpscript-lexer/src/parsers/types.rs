use crate::{
    Result,
    cx::ParseCx,
    err::Error,
    parsers::meta::{parse_def_flags, parse_def_meta},
    util::TokenCursor,
};
use dpscript_ast::prelude::types::{TypeRef, Typedef};
use dpscript_parser::{Keyword, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_typedef<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Typedef<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let flags = parse_def_flags(c, cx)?;

    c.expect_or_skip(Token::Keyword(Keyword::Typedef))?;

    let name = c.expect_ident()?;
    let span = c.end_span();

    Ok(Typedef {
        name,
        flags,
        meta,
        span,
    })
}

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_typeref<'a>(c: &mut TokenCursor<'a>, _cx: &mut ParseCx<'a>) -> Result<TypeRef<'a>> {
    let id = c.expect_ident().map_err(|_| Error::Skip)?;

    Ok(TypeRef {
        span: id.1,
        name: id,
        resolved: None,
    })
}
