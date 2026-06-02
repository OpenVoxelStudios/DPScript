use crate::{Result, cx::ParseCx, parsers::expr::parse_expr, util::TokenCursor};
use dpscript_ast::prelude::def::block::{Block, BlockKind};
use dpscript_parser::{BraceType, Keyword, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_block<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Block<'a>> {
    c.begin_span();

    let kind;
    let span;

    match c.take_next()? {
        (Token::Keyword(Keyword::Init), sp) => {
            kind = BlockKind::Init;
            span = sp;
        }

        (Token::Keyword(Keyword::Tick), sp) => {
            kind = BlockKind::Tick;
            span = sp;
        }

        _ => return Err(cx.skip()),
    }

    let mut block = c.expect_group(BraceType::Braces)?;
    let mut body = Vec::new();

    while block.has_next() {
        body.push(parse_expr(&mut block, cx)?);
    }

    block.assert_empty()?;

    Ok(Block {
        kind,
        body,
        span,
        meta: Default::default(),
    })
}
