use crate::{ParseCx, easy::parse_block, inner::Rule, parse_err, util::ParserUtil};
use ast::{
    attr::AttrNode,
    data::{SourceSpan, SpanUtil},
    func::{FuncFlags, FunctionArg, FunctionNode},
};
use miette::{Result, Severity};
use pest::iterators::Pairs;
use std::collections::BTreeMap;

pub fn parse_fn<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<FunctionNode<'a>> {
    let mut flags = FuncFlags::empty();

    while let Some(node) = inner.peek() {
        match node.as_rule() {
            Rule::_pub => flags |= FuncFlags::Public,
            Rule::_facade => flags |= FuncFlags::Facade,
            Rule::_inline => flags |= FuncFlags::Inline,
            Rule::_compiler => flags |= FuncFlags::Compiler,
            Rule::_operator => flags |= FuncFlags::Operator,
            Rule::_instance => flags |= FuncFlags::Instance,

            _ => break,
        }

        inner.next_checked(cx).unwrap();
    }

    let receiver = inner.next_type(cx).ok();
    let name = inner.next_ident(cx)?;
    let ident = cx.start_block();

    // Some functions can have zero arguments, and then pest omits the rule
    let args = if let Some(args) = inner.next_checked(cx) {
        if args.as_rule() != Rule::_fn_args {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::fn_args",
                labels = vec![args.as_span().label()],
                "Expected function arguments!"
            );
        }

        Some(parse_fn_args(
            cx,
            args.as_span().into(),
            &mut args.into_inner(),
        )?)
    } else {
        None
    }
    .unwrap_or_default();

    let ret = inner.next_type(cx).ok();

    let body = if let Some(block) = inner.next_checked(cx) {
        if block.as_rule() != Rule::_block {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::block",
                labels = vec![block.as_span().label()],
                "Expected a block!"
            );
        }

        Some(parse_block(
            cx,
            block.as_span().into(),
            &mut block.into_inner(),
        )?)
    } else {
        None
    }
    .unwrap_or_default();

    cx.end_block();

    // TODO: Attributes
    let attrs = BTreeMap::new();

    let id = attrs
        .get("name")
        .map(|it: &AttrNode<'a>| {
            it.values
                .first()
                .map(|it| it.as_literal().map(|it| it.data.as_string()))
        })
        .flatten()
        .flatten()
        .flatten()
        .unwrap_or(ident.path);

    let ident = cx.ident(id);

    Ok(FunctionNode {
        span,
        name,
        args,
        return_type: ret,
        body,
        flags,
        receiver,
        ident,
        attrs,
        scope: None,
    })
}

pub fn parse_fn_args<'a>(
    cx: &mut ParseCx<'a>,
    _span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<Vec<FunctionArg<'a>>> {
    let mut args = Vec::new();

    while let Some(node) = inner.next_checked(cx) {
        if node.as_rule() != Rule::_fn_arg {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::fn_arg",
                labels = vec![node.as_span().label()],
                "Expected a function argument!"
            );
        }

        args.push(parse_fn_arg(
            cx,
            node.as_span().into(),
            &mut node.into_inner(),
        )?);
    }

    Ok(args)
}

pub fn parse_fn_arg<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<FunctionArg<'a>> {
    let is_ref = inner.check_next(cx, Rule::_ref_kw).is_ok();
    let name = inner.next_ident(cx)?;
    let ty = inner.next_type(cx)?;
    let location = cx.local_var();

    Ok(FunctionArg {
        span,
        name,
        ty,
        is_ref,

        // TODO: Attributes
        is_this: false,
        attrs: BTreeMap::new(),
        location,
    })
}
