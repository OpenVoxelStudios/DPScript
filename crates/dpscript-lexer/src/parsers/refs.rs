use crate::{Result, cx::ParseCx, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::{
    HasSpan,
    value::refs::{ValueRef, VarRef},
};
use dpscript_parser::{Operator, Token};

pub fn parse_var_ref<'a>(c: &mut TokenCursor<'a>, _cx: &mut ParseCx<'a>) -> Result<'a, VarRef<'a>> {
    let name = c.expect_ident()?;

    Ok(VarRef {
        name,
        resolved: None,
        span: name.1,
    })
}

pub fn parse_value_ref<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<'a, ValueRef<'a>> {
    let root = parse_value(c, cx)?;
    let mut span = root.span();
    let mut path = Vec::new();

    c.expect(Token::Operator(Operator::Dot))?;

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
