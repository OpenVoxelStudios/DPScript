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
use dpscript_ast::prelude::{
    def::func::{Function, FunctionArg, FunctionInfo},
    types::{TypeRef, TypeRefData},
};
use dpscript_parser::{BraceType, Keyword, Operator, Punct, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
fn parse_fn_arg<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<FunctionArg<'a>> {
    c.begin_span();

    let meta = parse_def_meta(c, cx)?;
    let is_const = c.next_if_eq(&Token::Keyword(Keyword::Const)).is_some();
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
        is_const,
    })
}

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_func<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Function<'a>> {
    c.begin_span();

    let flags = parse_def_flags(c, cx)?;

    c.expect_or_skip(Token::Keyword(Keyword::Fn))?;

    let mut target = None;
    let mut name = c.expect_ident()?;

    if c.check(&Token::Operator(Operator::ModulePath)) {
        target = Some(TypeRef {
            span: name.1,
            data: TypeRefData::Named { name },
            resolved: None,
        });

        name = c.expect_ident()?;
    }

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

    if c.check(&Token::Operator(Operator::Returns)) {
        ret = Some(parse_typeref(c, cx)?);
    }

    if c.check(&Token::Punct(Punct::Semi)) {
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
