use crate::{
    Result,
    cx::ParseCx,
    parsers::{expr::parse_expr, value::parse_value},
    util::TokenCursor,
};
use dpscript_ast::prelude::expr::block::loops::ForLoop;
use dpscript_parser::{BraceType, Keyword, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_for_loop<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<ForLoop<'a>> {
    c.begin_span();
    c.expect_or_skip(Token::Keyword(Keyword::For))?;

    let var = c.expect_ident()?;

    c.expect(Token::Keyword(Keyword::In))?;

    let array = parse_value(c, cx)?;
    let mut block = c.expect_group(BraceType::Braces)?;
    let mut body = Vec::new();

    while block.has_next() {
        body.push(parse_expr(&mut block, cx)?);
    }

    block.assert_empty()?;

    let span = c.end_span();

    Ok(ForLoop {
        array,
        var,
        body,
        span,
    })
}
