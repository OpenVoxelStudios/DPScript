use crate::{Result, cx::ParseCx, err::Error, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::{
    HasSpan,
    value::refs::{ValueRef, VarRef},
};
use dpscript_parser::{Operator, Punct, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_var_ref<'a>(c: &mut TokenCursor<'a>, _cx: &mut ParseCx<'a>) -> Result<VarRef<'a>> {
    let name = c.expect_ident().map_err(|_| Error::Skip)?;

    Ok(VarRef {
        name,
        resolved: None,
        span: name.1,
    })
}

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_value_ref<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<ValueRef<'a>> {
    let mut buf = Vec::new();

    loop {
        // if we somehow reach the end of the file before encountering a dot, then we are not accessing any fields
        let tkn = c.take_next().map_err(|_| Error::Skip)?;

        if tkn.0 == Token::Punct(Punct::Semi) {
            return Err(cx.skip());
        }

        if tkn.0 == Token::Operator(Operator::Dot) {
            break;
        }

        buf.push(tkn);
    }

    let mut buf = TokenCursor::new(buf);
    let root = parse_value(&mut buf, cx)?;
    let mut span = root.span();
    let mut path = Vec::new();

    loop {
        let ident = c.expect_ident()?;

        span = span + ident.1;
        path.push(ident);

        if !c.next_if_eq(&Token::Operator(Operator::Dot)).is_some() {
            break;
        }
    }

    Ok(ValueRef {
        root: Box::new(root),
        path,
        span,
    })
}
