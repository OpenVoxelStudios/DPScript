use crate::{Result, cx::ParseCx, util::TokenCursor};
use dpscript_ast::prelude::def::import::{Import, PathRef};
use dpscript_parser::{Keyword, Literal, Operator, Punct, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_import<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Import<'a>> {
    c.begin_span();
    c.expect_or_skip(Token::Keyword(Keyword::Import))?;

    let mut path = Vec::new();
    let mut last_was_sep = true;

    while let Some(tkn) = c.next() {
        if tkn.0 == Token::Punct(Punct::Semi) {
            break;
        }

        match tkn.0 {
            Token::Literal(Literal::Identifier(id)) if last_was_sep => {
                path.push((id, tkn.1));
                last_was_sep = false;
            }

            Token::Operator(Operator::ModulePath) if !last_was_sep => {
                last_was_sep = true;
            }

            other => return Err(cx.unexpected((other, tkn.1))),
        }
    }

    // TODO: Group imports

    let span = c.end_span();

    Ok(Import {
        paths: vec![PathRef { span, parts: path }],
        meta: Default::default(),
        span,
    })
}
