use std::collections::BTreeMap;

use ast::{
    data::{SourceSpan, SpanUtil},
    literal::{LiteralData, LiteralNode},
    nbt::{NbtValue, NbtValueData},
};
use miette::{Result, Severity};
use pest::iterators::Pairs;

use crate::{
    ParseCx,
    inner::Rule,
    parse_err,
    parser::parse_next,
    util::{ParserUtil, next_or_die, only_one},
};

pub fn parse_nbt<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<LiteralNode<'a>> {
    Ok(LiteralNode {
        span,
        data: LiteralData::Nbt(NbtValue {
            span,
            data: NbtValueData::Map(parse_nbt_obj(cx, span, inner)?),
        }),
    })
}

pub fn parse_nbt_obj<'a>(
    cx: &mut ParseCx<'a>,
    _span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<BTreeMap<&'a str, NbtValue<'a>>> {
    let mut map = BTreeMap::new();

    while !(*inner).is_empty() {
        let kv = next_or_die(cx, inner)?;

        if kv.as_rule() != Rule::_nbt_kv {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::nbt_kv",
                labels = vec![kv.as_span().label()],
                "Expected an NBT key-value item!"
            );
        }

        let mut kv = kv.into_inner();
        let key = kv.next_ident(cx)?;
        let value = next_or_die(cx, &mut kv)?;
        let span = value.as_span().into();

        let value = match value.as_rule() {
            Rule::_nbt_arr => {
                NbtValueData::Array(parse_nbt_arr(cx, span, &mut value.into_inner())?)
            }

            Rule::_nbt_obj => NbtValueData::Map(parse_nbt_obj(cx, span, &mut value.into_inner())?),

            _ => {
                let vec = parse_next(cx, &mut Pairs::single(value))?;

                NbtValueData::Expr(Box::new(only_one(cx, vec)?))
            }
        };

        map.insert(key.0, NbtValue { span, data: value });
    }

    Ok(map)
}

pub fn parse_nbt_arr<'a>(
    cx: &mut ParseCx<'a>,
    _span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<Vec<NbtValue<'a>>> {
    let mut arr = Vec::new();

    while !(*inner).is_empty() {
        let value = next_or_die(cx, inner)?;
        let span = value.as_span().into();

        let value = match value.as_rule() {
            Rule::_nbt_arr => {
                NbtValueData::Array(parse_nbt_arr(cx, span, &mut value.into_inner())?)
            }

            Rule::_nbt_obj => NbtValueData::Map(parse_nbt_obj(cx, span, &mut value.into_inner())?),

            _ => {
                let vec = parse_next(cx, &mut Pairs::single(value))?;

                NbtValueData::Expr(Box::new(only_one(cx, vec)?))
            }
        };

        arr.push(NbtValue { span, data: value });
    }

    Ok(arr)
}
