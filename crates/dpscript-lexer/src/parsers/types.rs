use crate::{
    Result,
    cx::ParseCx,
    err::Error,
    parsers::{
        meta::{parse_def_flags, parse_def_meta},
        value::parse_value,
    },
    util::TokenCursor,
};
use dpscript_ast::prelude::types::{TypeRef, TypeRefData, Typedef};
use dpscript_parser::{BraceType, Keyword, Punct, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_typedef<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Typedef<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let flags = parse_def_flags(c, cx)?;

    c.expect_or_skip(Token::Keyword(Keyword::Typedef))?;

    let name = c.expect_ident()?;

    c.expect(Token::Punct(Punct::Semi))?;

    let span = c.end_span();

    Ok(Typedef {
        name,
        flags,
        meta,
        span,
    })
}

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_typeref_id<'a>(c: &mut TokenCursor<'a>, _cx: &mut ParseCx<'a>) -> Result<TypeRef<'a>> {
    let id = c.expect_ident().map_err(|_| Error::Skip)?;

    Ok(TypeRef {
        span: id.1,
        data: TypeRefData::Named { name: id },
        resolved: None,
    })
}

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_typeref_arr<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<TypeRef<'a>> {
    c.begin_span();

    let mut group = c
        .expect_group(BraceType::Brackets)
        .map_err(|_| Error::Skip)?;

    let inner = parse_typeref(&mut group, cx)?;

    if group.check(&Token::Punct(Punct::Semi)) {
        let length = parse_value(&mut group, cx)?;

        group.assert_empty()?;

        Ok(TypeRef {
            span: c.end_span(),
            data: TypeRefData::SizedArray {
                inner: Box::new(inner),
                length: Box::new(length),
            },
            resolved: None,
        })
    } else {
        group.assert_empty()?;

        Ok(TypeRef {
            span: c.end_span(),
            data: TypeRefData::UnsizedArray {
                inner: Box::new(inner),
            },
            resolved: None,
        })
    }
}

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_typeref<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<TypeRef<'a>> {
    c.save();

    match parse_typeref_id(c, cx) {
        Ok(it) => return Ok(it),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_typeref_arr(c, cx) {
        Ok(it) => return Ok(it),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    Err(cx.unexpected(c.take_next()?))
}
