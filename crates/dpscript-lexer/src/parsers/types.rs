use crate::{
    Result,
    cx::ParseCx,
    parsers::meta::{parse_def_flags, parse_def_meta},
    util::TokenCursor,
};
use dpscript_ast::prelude::types::{TypeRef, Typedef};
use dpscript_parser::{Keyword, Token};

pub fn parse_typedef<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<'a, Typedef<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let flags = parse_def_flags(c, cx)?;

    c.expect(Token::Keyword(Keyword::Typedef))?;

    let name = c.expect_ident()?;
    let span = c.end_span();

    Ok(Typedef {
        name,
        flags,
        meta,
        span,
    })
}

pub fn parse_typeref<'a>(
    c: &mut TokenCursor<'a>,
    _cx: &mut ParseCx<'a>,
) -> Result<'a, TypeRef<'a>> {
    let id = c.expect_ident()?;

    Ok(TypeRef {
        span: id.1,
        name: id,
        resolved: None,
    })
}
