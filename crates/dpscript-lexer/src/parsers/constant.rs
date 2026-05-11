use crate::{
    Result,
    cx::ParseCx,
    parsers::{meta::parse_def_flags, types::parse_typeref, value::parse_value},
    util::TokenCursor,
};
use dpscript_ast::prelude::def::constant::Constant;
use dpscript_parser::{Assignment, Keyword, Punct, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_constant<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Constant<'a>> {
    c.begin_span();

    let flags = parse_def_flags(c, cx)?;

    c.expect_or_skip(Token::Keyword(Keyword::Const))?;

    let name = c.expect_ident()?;

    c.expect(Token::Punct(Punct::Colon))?;

    let ty = parse_typeref(c, cx)?;

    c.expect(Token::Assignment(Assignment::Equal))?;

    let value = parse_value(c, cx)?;

    c.expect(Token::Punct(Punct::Semi))?;

    let span = c.end_span();

    Ok(Constant {
        name,
        meta: Default::default(),
        flags,
        span,
        ty,
        value,
    })
}
