use crate::{
    Result,
    cx::ParseCx,
    parsers::{expr::parse_expr, value::parse_value},
    util::TokenCursor,
};
use dpscript_ast::prelude::expr::block::at::At;
use dpscript_parser::{BraceType, Keyword, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_at_block<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<At<'a>> {
    c.begin_span();
    c.expect_or_skip(Token::Keyword(Keyword::At))?;

    let arg = parse_value(c, cx)?;
    let mut block = c.expect_group(BraceType::Braces)?;
    let mut body = Vec::new();

    while block.has_next() {
        body.push(parse_expr(&mut block, cx)?);
    }

    block.assert_empty()?;

    let span = c.end_span();

    Ok(At { arg, body, span })
}
