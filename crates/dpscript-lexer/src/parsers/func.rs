use crate::{
    Result,
    cx::ParseCx,
    parsers::{
        expr::parse_expr,
        meta::{parse_def_flags, parse_def_meta},
        types::parse_typeref,
    },
    util::TokenCursor,
};
use dpscript_ast::prelude::def::func::{Function, FunctionArg, FunctionInfo};
use dpscript_parser::{BraceType, Keyword, Operator, Punct, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
fn parse_fn_arg<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<FunctionArg<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let is_ref = c.next_if_eq(&Token::Keyword(Keyword::Ref)).is_some();
    let name = c.expect_ident()?;

    c.expect(Token::Punct(Punct::Colon))?;

    let ty = parse_typeref(c, cx)?;
    let span = c.end_span();

    Ok(FunctionArg {
        name,
        ty,
        span,
        meta,
        is_ref,
    })
}

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_func<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Function<'a>> {
    c.begin_span();

    let flags = parse_def_flags(c, cx)?;

    c.expect_or_skip(Token::Keyword(Keyword::Fn))?;

    let mut target = None;

    if c.peek_in(2)
        .is_some_and(|it| *it == Token::Operator(Operator::ModulePath))
    {
        target = Some(parse_typeref(c, cx)?);
        c.expect(Token::Operator(Operator::ModulePath))?;
    }

    let name = c.expect_ident()?;
    let mut group = c.expect_group(BraceType::Parens)?;
    let mut args = Vec::new();

    while group.has_next() {
        args.push(parse_fn_arg(&mut group, cx)?);

        if !group.next_if_eq(&Token::Punct(Punct::Comma)).is_some() {
            break;
        }
    }

    group.assert_empty()?;

    let mut ret = None;

    if c.next_if_eq(&Token::Operator(Operator::Returns)).is_some() {
        ret = Some(parse_typeref(c, cx)?);
    }

    if c.next_if_eq(&Token::Punct(Punct::Semi)).is_some() {
        let span = c.end_span();

        return Ok(Function {
            info: FunctionInfo {
                name,
                flags,
                target,
                args,
                ret,
                span,
                meta: Default::default(),
            },
            body: Vec::new(),
            span,
        });
    }

    let mut block = c.expect_group(BraceType::Braces)?;
    let mut body = Vec::new();

    while block.has_next() {
        body.push(parse_expr(&mut block, cx)?);
    }

    block.assert_empty()?;

    let span = c.end_span();

    Ok(Function {
        info: FunctionInfo {
            name,
            flags,
            target,
            args,
            ret,
            span,
            meta: Default::default(),
        },
        body,
        span,
    })
}
