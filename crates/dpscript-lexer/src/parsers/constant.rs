use crate::{
    Result,
    cx::ParseCx,
    parsers::{
        meta::{parse_def_flags, parse_def_meta},
        types::parse_typeref,
        value::parse_value,
    },
    util::TokenCursor,
};
use dpscript_ast::prelude::def::constant::Constant;
use dpscript_parser::{Assignment, Keyword, Punct, Token};

pub fn parse_constant<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<'a, Constant<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let flags = parse_def_flags(c, cx)?;

    c.expect(Token::Keyword(Keyword::Const))?;

    let name = c.expect_ident()?;

    c.expect(Token::Punct(Punct::Colon))?;

    let ty = parse_typeref(c, cx)?;

    c.expect(Token::Assignment(Assignment::Equal))?;

    let value = parse_value(c, cx)?;
    let span = c.end_span();

    Ok(Constant {
        name,
        meta,
        flags,
        span,
        ty,
        value,
    })
}
