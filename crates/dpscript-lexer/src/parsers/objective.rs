use crate::{
    Result,
    cx::ParseCx,
    parsers::meta::{parse_def_flags, parse_def_meta},
    util::TokenCursor,
};
use dpscript_ast::prelude::def::objective::Objective;
use dpscript_parser::{Assignment, Keyword, Token};

pub fn parse_objective<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<'a, Objective<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let flags = parse_def_flags(c, cx)?;

    c.expect(Token::Keyword(Keyword::Objective))?;

    let name = c.expect_ident()?;

    c.expect(Token::Assignment(Assignment::Equal))?;

    let criteria = c.expect_str()?;
    let span = c.end_span();

    Ok(Objective {
        name,
        criteria,
        span,
        meta,
        flags,
    })
}
