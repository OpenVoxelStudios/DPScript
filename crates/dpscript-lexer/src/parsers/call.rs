use crate::{Result, cx::ParseCx, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::expr::call::Call;
use dpscript_parser::{BraceType, Punct, Token};

pub fn parse_call<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<'a, Call<'a>> {
    c.begin_span();

    let target = parse_value(c, cx)?;
    let mut group = c.expect_group(BraceType::Parens)?;
    let mut args = Vec::new();

    while group.has_next() {
        args.push(parse_value(&mut group, cx)?);

        if !group.next_if_eq(&Token::Punct(Punct::Comma)).is_some() {
            break;
        }
    }

    let span = c.end_span();

    Ok(Call {
        args,
        resolved: None,
        span,
        target: Box::new(target),
    })
}
