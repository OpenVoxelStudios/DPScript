use crate::{Result, cx::ParseCx, err::Error, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::expr::call::Call;
use dpscript_parser::{BraceType, Punct, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_call<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Call<'a>> {
    c.begin_span();

    let mut buf = Vec::new();
    let mut args_buf;

    loop {
        // if we somehow reach the end of the file before finding the arg list, then we are not calling anything
        let tkn = c.take_next().map_err(|_| Error::Skip)?;

        if tkn.0 == Token::Punct(Punct::Semi) {
            return Err(cx.skip());
        }

        if let Token::BraceGroup(BraceType::Parens, buf) = tkn.0 {
            args_buf = TokenCursor::new(buf.into_inner());
            break;
        }

        buf.push(tkn);
    }

    let mut buf = TokenCursor::new(buf);
    let target = parse_value(&mut buf, cx)?;
    let mut args = Vec::new();

    while args_buf.has_next() {
        args.push(parse_value(&mut args_buf, cx)?);

        if !args_buf.next_if_eq(&Token::Punct(Punct::Comma)).is_some() {
            break;
        }
    }

    args_buf.assert_empty()?;

    let span = c.end_span();

    Ok(Call {
        args,
        resolved: None,
        span,
        target: Box::new(target),
    })
}
