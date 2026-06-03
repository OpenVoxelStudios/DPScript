use crate::{
    Result,
    cx::ParseCx,
    parsers::{expr::parse_expr, value::parse_value},
    util::TokenCursor,
};
use dpscript_ast::prelude::expr::block::cond::{Cond, Condition};
use dpscript_parser::{BraceType, Keyword, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_cond<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Cond<'a>> {
    c.begin_span();

    let mut conds = Vec::new();
    let mut else_block = Vec::new();

    c.begin_span();
    c.expect_or_skip(Token::Keyword(Keyword::If))?;

    let condition = parse_value(c, cx)?;
    let mut block = c.expect_group(BraceType::Braces)?;
    let mut body = Vec::new();

    while block.has_next() {
        body.push(parse_expr(&mut block, cx)?);
    }

    block.assert_empty()?;

    let span = c.end_span();

    conds.push(Condition {
        body,
        condition,
        span,
        scope: None,
    });

    while c.next_if_eq(&Token::Keyword(Keyword::Else)).is_some() {
        c.begin_span_prev();

        if c.next_if_eq(&Token::Keyword(Keyword::If)).is_some() {
            let condition = parse_value(c, cx)?;
            let mut block = c.expect_group(BraceType::Braces)?;
            let mut body = Vec::new();

            while block.has_next() {
                body.push(parse_expr(&mut block, cx)?);
            }

            block.assert_empty()?;

            let span = c.end_span();

            conds.push(Condition {
                body,
                condition,
                span,
                scope: None,
            });
        } else {
            let mut block = c.expect_group(BraceType::Braces)?;

            while block.has_next() {
                else_block.push(parse_expr(&mut block, cx)?);
            }

            block.assert_empty()?;
            c.end_span();

            break;
        }
    }

    let span = c.end_span();

    Ok(Cond {
        conditions: conds,
        else_block,
        span,
        else_scope: None,
    })
}
