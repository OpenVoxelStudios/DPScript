use crate::{Result, cx::ParseCx, err::Error, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::value::{
    arr::ArrayLiteral,
    literal::{DslLiteral, DslMarker, Literal, LiteralValue},
    nbt::NbtLiteral,
};
use dpscript_parser::{BraceType, Keyword, Literal as TLiteral, Operator, Punct, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_literal<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Literal<'a>> {
    match c.next() {
        Some((Token::Literal(TLiteral::Byte(it)), span)) => Ok(Literal {
            value: LiteralValue::Byte(it),
            span,
        }),

        Some((Token::Literal(TLiteral::Double(it)), span)) => Ok(Literal {
            value: LiteralValue::Double(it),
            span,
        }),

        Some((Token::Literal(TLiteral::Float(it)), span)) => Ok(Literal {
            value: LiteralValue::Float(it),
            span,
        }),

        Some((Token::Literal(TLiteral::Int(it)), span)) => Ok(Literal {
            value: LiteralValue::Int(it),
            span,
        }),

        Some((Token::Literal(TLiteral::Long(it)), span)) => Ok(Literal {
            value: LiteralValue::Long(it),
            span,
        }),

        Some((Token::Literal(TLiteral::String(it)), span)) => Ok(Literal {
            value: LiteralValue::String(it),
            span,
        }),

        Some((Token::Keyword(Keyword::True), span)) => Ok(Literal {
            value: LiteralValue::Bool(true),
            span,
        }),

        Some((Token::Keyword(Keyword::False), span)) => Ok(Literal {
            value: LiteralValue::Bool(false),
            span,
        }),

        Some(_) => Err(cx.skip()),
        None => Err(cx.eof(c.cur_span())),
    }
}

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_dsl_literal<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<DslLiteral<'a>> {
    c.begin_span();

    let marker = match c.take_next()? {
        (Token::Operator(Operator::At), _) => DslMarker::At,
        (Token::Punct(Punct::Hash), _) => DslMarker::Hash,
        _ => return Err(cx.skip()),
    };

    let value = parse_literal(c, cx)?;

    Ok(DslLiteral {
        dsl_marker: marker,
        value: value.value,
        span: c.end_span(),
    })
}

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_nbt_literal<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<NbtLiteral<'a>> {
    c.begin_span();

    let mut group = c.expect_group(BraceType::Braces).map_err(|_| Error::Skip)?;
    let mut values = Vec::new();

    while group.has_next() {
        let name = group.expect_ident()?;

        group.expect(Token::Punct(Punct::Colon))?;

        let value = parse_value(&mut group, cx)?;

        values.push((name, value));

        if !group.next_if_eq(&Token::Punct(Punct::Comma)).is_some() {
            break;
        }
    }

    group.assert_empty()?;

    Ok(NbtLiteral {
        span: c.end_span(),
        values,
    })
}

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_array_literal<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<ArrayLiteral<'a>> {
    c.begin_span();

    let mut group = c
        .expect_group(BraceType::Brackets)
        .map_err(|_| Error::Skip)?;

    let mut values = Vec::new();

    while group.has_next() {
        values.push(parse_value(&mut group, cx)?);

        if !group.next_if_eq(&Token::Punct(Punct::Comma)).is_some() {
            break;
        }
    }

    group.assert_empty()?;

    Ok(ArrayLiteral {
        span: c.end_span(),
        values,
    })
}
