use std::collections::BTreeMap;

use crate::{
    ParseCx,
    inner::Rule,
    parse_err,
    parser::parse_next,
    util::{ParserUtil, next_or_die, only_one},
};
use ast::{
    at::AtNode, block::{BlockKind, BlockNode}, call::CallNode, cond::ConditionalNode, constant::ConstantNode, data::{SourceSpan, SpanUtil}, literal::{LiteralData, LiteralNode}, nbt::{NbtValue, NbtValueData}, node::Node, refs::{RefData, RefNode}, ret::ReturnNode, special::{SpecialData, SpecialNode}, unop::{UnaryOpNode, UnaryOperation}, var::VarNode
};
use miette::{IntoDiagnostic, Result, Severity};
use pest::iterators::Pairs;

pub fn parse_inner_const<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<ConstantNode<'a>> {
    let name = inner.next_ident(cx)?;
    let ty = inner.next_type(cx).ok();
    let (mut value, _) = inner.next_expr(cx)?;
    let value = parse_next(cx, &mut value)?;
    let value = only_one(cx, value)?;

    Ok(ConstantNode {
        is_public: false,
        span,
        name,
        ty,
        value: Box::new(value),
        keep: false, // TODO: #[keep] attribute
    })
}

pub fn parse_top_const<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<ConstantNode<'a>> {
    let is_public = inner.check_next(cx, Rule::_pub).is_ok();
    let mut pair = inner.one_inner(cx)?;
    let mut node = parse_inner_const(cx, span, &mut pair.0)?;

    node.is_public = is_public;

    Ok(node)
}

pub fn parse_selector<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<SpecialNode<'a>> {
    let (sel, _) = inner.next_str(cx)?;

    Ok(SpecialNode {
        span,
        data: SpecialData::Selector(sel),
    })
}

pub fn parse_component<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<SpecialNode<'a>> {
    let (txt, txt_span) = inner.next_str(cx)?;
    let mut map = BTreeMap::new();

    map.insert(
        "text",
        NbtValue {
            span: txt_span,
            data: NbtValueData::String(txt),
        },
    );

    Ok(SpecialNode {
        span,
        data: SpecialData::Component(NbtValue {
            span,
            data: NbtValueData::Map(map),
        }),
    })
}

pub fn parse_pos<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<SpecialNode<'a>> {
    let x = inner.parse_one_next(cx)?;
    let y = inner.parse_one_next(cx)?;
    let z = inner.parse_one_next(cx)?;

    Ok(SpecialNode {
        span,
        data: SpecialData::Pos(Box::new(x), Box::new(y), Box::new(z)),
    })
}

pub fn parse_at_block<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<AtNode<'a>> {
    let pos = inner.parse_one_next(cx)?;
    let body = inner.parse_next(cx)?;

    Ok(AtNode {
        span,
        pos: Box::new(pos),
        body,
    })
}

pub fn parse_return<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<ReturnNode<'a>> {
    // FIXME: Throw an error if this isn't an expr!
    //        Right now it just replaces the value with `None` since it
    //        only checks for an expr, not the length of `inner` entriely.

    let value = if let Ok((mut next, _)) = inner.next_expr(cx) {
        let parsed = parse_next(cx, &mut next)?;

        Some(Box::new(only_one(cx, parsed)?))
    } else {
        None
    };

    Ok(ReturnNode { span, value })
}

pub fn parse_init_block<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<BlockNode<'a>> {
    Ok(BlockNode {
        span,
        body: parse_block(cx, span, inner)?,
        kind: BlockKind::Init,

        // TODO: Attributes
        attrs: BTreeMap::new(),
        keep: false,
    })
}

pub fn parse_tick_block<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<BlockNode<'a>> {
    Ok(BlockNode {
        span,
        body: parse_block(cx, span, inner)?,
        kind: BlockKind::Tick,

        // TODO: Attributes
        attrs: BTreeMap::new(),
        keep: false,
    })
}

pub fn parse_block<'a>(
    cx: &mut ParseCx<'a>,
    _span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<Vec<Node<'a>>> {
    let mut nodes = Vec::new();

    while !(*inner).is_empty() {
        nodes.extend(parse_next(cx, inner)?)
    }

    Ok(nodes)
}

pub fn parse_ref<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<RefNode<'a>> {
    let lhs = inner.parse_one_next(cx)?;
    let rhs = next_or_die(cx, inner)?;

    let data = match rhs.as_rule() {
        Rule::_ref_index => RefData::ArrayIndex(Box::new(rhs.into_inner().parse_one_next(cx)?)),
        Rule::_ident => RefData::Ident(rhs.as_str()),

        _ => {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::index_or_ident",
                labels = vec![rhs.as_span().label()],
                "Expected an array index or an identifier!"
            );
        }
    };

    Ok(RefNode {
        span,
        lhs: Box::new(lhs),
        data,
    })
}

pub fn parse_ident<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<LiteralNode<'a>> {
    let (ident, _) = inner.next_ident(cx)?;

    Ok(LiteralNode {
        span,
        data: LiteralData::Ident(ident),
    })
}

pub fn parse_int<'a>(
    _cx: &mut ParseCx<'a>,
    span: SourceSpan,
    s: &'a str,
) -> Result<LiteralNode<'a>> {
    Ok(LiteralNode {
        span,
        data: LiteralData::Int(s.parse().into_diagnostic()?),
    })
}

pub fn parse_double<'a>(
    _cx: &mut ParseCx<'a>,
    span: SourceSpan,
    s: &'a str,
) -> Result<LiteralNode<'a>> {
    Ok(LiteralNode {
        span,
        data: LiteralData::Double(s.parse().into_diagnostic()?),
    })
}

pub fn parse_float<'a>(
    _cx: &mut ParseCx<'a>,
    span: SourceSpan,
    s: &'a str,
) -> Result<LiteralNode<'a>> {
    Ok(LiteralNode {
        span,
        data: LiteralData::Float(s.parse().into_diagnostic()?),
    })
}

pub fn parse_str<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<LiteralNode<'a>> {
    let (value, _) = inner.next_str(cx)?;

    Ok(LiteralNode {
        span,
        data: LiteralData::String(value),
    })
}

pub fn parse_bool<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<LiteralNode<'a>> {
    let pair = next_or_die(cx, inner)?;

    Ok(LiteralNode {
        span,
        data: LiteralData::Bool(pair.as_rule() == Rule::_true),
    })
}

pub fn parse_var<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<VarNode<'a>> {
    let name = inner.next_ident(cx)?;
    let ty = inner.next_type(cx).ok();
    let value = inner.parse_one_next(cx).ok().map(Box::new);

    Ok(VarNode {
        span,
        is_arg: false,
        name,
        ty,
        value,
    })
}

pub fn parse_call<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<CallNode<'a>> {
    let recv = inner.parse_one_next(cx)?;
    let mut args = Vec::new();

    while !(*inner).is_empty() {
        args.extend(parse_next(cx, inner)?);
    }

    Ok(CallNode {
        span,
        receiver: Box::new(recv),
        args,
    })
}

pub fn parse_arr<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<LiteralNode<'a>> {
    let mut items = Vec::new();

    while !(*inner).is_empty() {
        items.extend(parse_next(cx, inner)?);
    }

    Ok(LiteralNode {
        span,
        data: LiteralData::Array(items),
    })
}

pub fn parse_if<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<ConditionalNode<'a>> {
    let recv = inner.parse_one_next(cx)?;
    let body = parse_block(cx, span, inner)?;

    // TODO: Else ifs

    Ok(ConditionalNode {
        span,
        condition: Box::new(recv),
        body,

        // TODO: else & else if
        else_body: vec![],
        else_ifs: vec![],
    })
}

pub fn parse_expr<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<Node<'a>> {
    if inner.len() > 1 {
        let next = next_or_die(cx, inner)?;

        let op = match next.as_rule() {
            Rule::_un_op_neg => UnaryOperation::Negate,
            Rule::_un_op_not => UnaryOperation::Invert,
            Rule::_un_op_pos => UnaryOperation::None,
            Rule::_un_op_rel => UnaryOperation::LocalOffset,

            _ => parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::unary_op",
                labels = vec![next.as_span().label()],
                "Expected a unary operator!"
            ),
        };

        Ok(Node::UnaryOp(UnaryOpNode {
            op,
            span,
            value: Box::new(inner.parse_one_next(cx)?),
        }))
    } else {
        inner.parse_one_next(cx)
    }
}
