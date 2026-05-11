use crate::{Result, cx::ParseCx, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::expr::ret::Return;
use dpscript_parser::{Keyword, Punct, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_ret<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Return<'a>> {
    c.begin_span();
    c.expect_or_skip(Token::Keyword(Keyword::Return))?;

    if c.next_if_eq(&Token::Punct(Punct::Semi)).is_some() {
        Ok(Return {
            value: None,
            span: c.end_span(),
        })
    } else {
        let value = parse_value(c, cx)?;

        Ok(Return {
            value: Some(value),
            span: c.end_span(),
        })
    }
}
