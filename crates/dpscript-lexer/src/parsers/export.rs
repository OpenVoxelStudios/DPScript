use crate::{Result, cx::ParseCx, util::TokenCursor};
use dpscript_ast::prelude::def::{export::Export, import::PathRef};
use dpscript_parser::{Keyword, Literal, Operator, Punct, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_export<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Export<'a>> {
    c.begin_span();
    c.expect_or_skip(Token::Keyword(Keyword::Export))?;

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

            Token::Operator(Operator::Star) => {
                break;
            }

            other => return Err(cx.unexpected((other, tkn.1))),
        }
    }

    c.expect(Token::Punct(Punct::Semi))?;

    // TODO: Group exports

    let span = c.end_span();

    Ok(Export {
        paths: vec![PathRef { span, parts: path }],
        meta: Default::default(),
        span,
    })
}
