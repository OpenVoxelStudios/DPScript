use crate::{Result, cx::ParseCx, parsers::meta::parse_def_flags, util::TokenCursor};
use dpscript_ast::prelude::def::objective::Objective;
use dpscript_parser::{Assignment, Keyword, Punct, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_objective<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Objective<'a>> {
    c.begin_span();

    let flags = parse_def_flags(c, cx)?;

    c.expect_or_skip(Token::Keyword(Keyword::Objective))?;

    let name = c.expect_ident()?;

    c.expect(Token::Assignment(Assignment::Equal))?;

    let criteria = c.expect_str()?;

    c.expect(Token::Punct(Punct::Semi))?;

    let span = c.end_span();

    Ok(Objective {
        name,
        criteria,
        span,
        meta: Default::default(),
        flags,
    })
}
