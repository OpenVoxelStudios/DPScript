use crate::{
    Result,
    cx::ParseCx,
    parsers::{types::parse_typeref, value::parse_value},
    util::TokenCursor,
};
use dpscript_ast::prelude::expr::var::Variable;
use dpscript_parser::{Assignment, Keyword, Punct, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_var<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Variable<'a>> {
    c.begin_span();
    c.expect_or_skip(Token::Keyword(Keyword::Let))?;

    let name = c.expect_ident()?;
    let mut ty = None;
    let mut value = None;

    if c.next_if_eq(&Token::Punct(Punct::Colon)).is_some() {
        ty = Some(parse_typeref(c, cx)?);
    }

    if c.next_if_eq(&Token::Assignment(Assignment::Equal))
        .is_some()
    {
        value = Some(parse_value(c, cx)?);
    }

    c.expect(Token::Punct(Punct::Semi))?;

    let span = c.end_span();

    Ok(Variable {
        name,
        ty,
        value,
        span,
    })
}
