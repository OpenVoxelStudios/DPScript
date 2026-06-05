use std::marker::PhantomData;

use crate::{
    Result,
    cx::ParseCx,
    err::Error,
    parsers::{literal::parse_literal, types::parse_typeref},
    util::TokenCursor,
};
use dpscript_ast::prelude::{
    meta::{AllowFlag, BuiltinInfo, DefFlags, DefMeta, EnforceType, Repr, Require, Restrict},
    value::literal::DslMarker,
};
use dpscript_core::{MaxBy, MinBy, Spanned};
use dpscript_parser::{Assignment, BraceType, Keyword, Literal, Punct, Token};

#[derive(Debug)]
pub struct DefFlagsBuilder<'a> {
    inner: Vec<DefFlags>,
    _ty: PhantomData<&'a ()>,
}

impl<'a> DefFlagsBuilder<'a> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            _ty: PhantomData::default(),
        }
    }

    pub fn try_push(
        &mut self,
        cx: &ParseCx<'a>,
        token: Spanned<Token<'a>>,
        item: DefFlags,
    ) -> Result<()> {
        if !self.inner.contains(&item) {
            self.inner.push(item);
            Ok(())
        } else {
            Err(cx.unexpected(token))
        }
    }

    pub fn build(self) -> Vec<DefFlags> {
        self.inner
    }
}

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_def_flags<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Vec<DefFlags>> {
    let mut flags = DefFlagsBuilder::new();

    loop {
        match c.peek().copied() {
            Some(Token::Keyword(Keyword::Pub)) => {
                flags.try_push(cx, c.take_next()?, DefFlags::Public)?
            }

            Some(Token::Keyword(Keyword::Const))
                if c.peek()
                    .is_some_and(|it| *it == Token::Keyword(Keyword::Fn)) =>
            {
                flags.try_push(cx, c.take_next()?, DefFlags::Const)?
            }

            Some(Token::Keyword(Keyword::Operator)) => {
                flags.try_push(cx, c.take_next()?, DefFlags::Operator)?
            }

            _ => break,
        }
    }

    c.clear_peek();

    Ok(flags.build())
}

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_single_def_meta<'a>(
    c: &mut TokenCursor<'a>,
    cx: &mut ParseCx<'a>,
) -> Result<DefMeta<'a>> {
    c.clear_peek();
    c.expect(Token::Punct(Punct::Hash))?;
    c.begin_span_prev();

    let mut inner = c.expect_group(BraceType::Brackets)?;
    let mut meta = DefMeta::default();

    if inner.next_if_ident("builtin").is_some() {
        // #[builtin] / #[builtin()]
        // #[builtin(cast)]
        // #[builtin(convert_to = ...)]
        let mut info = BuiltinInfo::Generic;

        if let Ok(mut group) = inner.expect_group(BraceType::Parens) {
            if group.next_if_ident("cast").is_some() {
                info = BuiltinInfo::Cast;
                group.assert_empty()?;
            } else if group.next_if_ident("convert_to").is_some() {
                group.expect(Token::Assignment(Assignment::Equal))?;
                info = BuiltinInfo::ConvertTo(parse_typeref(&mut group, cx)?);
                group.assert_empty()?;
            } else {
                group.assert_empty()?;
            }
        }

        meta.builtin = Some(info);
    } else if inner.next_if_ident("inline").is_some() {
        // #[inline]
        meta.inline = true;
    } else if inner.next_if_ident("allow").is_some() {
        // #[allow(...)]
        let mut group = inner.expect_group(BraceType::Parens)?;
        let mut flags = Vec::new();

        loop {
            if group.next_if_ident("incomplete").is_some() {
                flags.push(AllowFlag::Incomplete);
            }

            if !group.next_if_eq(&Token::Punct(Punct::Comma)).is_some() {
                break;
            }
        }

        group.assert_empty()?;
        meta.allow = flags;
    } else if inner.next_if_ident("nullable").is_some() {
        // #[nullable]
        meta.nullable = true;
    } else if inner.next_if_ident("hint").is_some() {
        // #[hint = "..."]
        inner.expect(Token::Assignment(Assignment::Equal))?;
        meta.hint = Some(inner.expect_str()?);
    } else if inner.next_if_ident("restrict").is_some() {
        // #[restrict(...)]
        let mut group = inner.expect_group(BraceType::Parens)?;
        let mut restrict = Vec::new();

        if group.next_if_ident("values").is_some() {
            group.expect(Token::Assignment(Assignment::Equal))?;

            let mut list = group.expect_group(BraceType::Brackets)?;
            let mut fields = Vec::new();

            while list.has_next() {
                fields.push(parse_literal(&mut list, cx)?);

                if !list.next_if_eq(&Token::Punct(Punct::Comma)).is_some() {
                    break;
                }
            }

            list.assert_empty()?;
            restrict.push(Restrict::Values(fields));
        }

        group.assert_empty()?;
        meta.restrict = restrict;
    } else if inner.next_if_ident("require").is_some() {
        // #[require(...)]
        let mut group = inner.expect_group(BraceType::Parens)?;
        let mut require = Vec::new();

        if group.next_if_ident("one_of").is_some() {
            group.expect(Token::Assignment(Assignment::Equal))?;

            let mut list = group.expect_group(BraceType::Brackets)?;
            let mut fields = Vec::new();

            while let Ok((tkn, span)) = list.take_next() {
                if let Token::Literal(Literal::Identifier(id)) = tkn {
                    fields.push((id, span));
                } else {
                    return Err(cx.unexpected((tkn, span)));
                }

                if !list.check(&Token::Punct(Punct::Comma)) {
                    break;
                }
            }

            list.assert_empty()?;
            require.push(Require::OneOf(fields));
        } else if group.next_if_ident("store").is_some() {
            require.push(Require::Store);
        }

        group.assert_empty()?;
        meta.require = require;
    } else if inner.next_if_ident("dsl").is_some() {
        // #[dsl(...)]
        let mut group = inner.expect_group(BraceType::Parens)?;

        if group.next_if_ident("prefix").is_some() {
            group.expect(Token::Assignment(Assignment::Equal))?;

            let it = group.take_next()?;

            let Token::Literal(Literal::String(s)) = it.0 else {
                return Err(cx.unexpected(it));
            };

            match s {
                "#" => meta.dsl = Some(DslMarker::Hash),
                "@" => meta.dsl = Some(DslMarker::At),

                _ => return Err(cx.unexpected(it)),
            }
        }

        group.assert_empty()?;
    } else if inner.next_if_ident("cmd").is_some() {
        // #[cmd(...)]
        let mut group = inner.expect_group(BraceType::Parens)?;
        let cmd = group.expect_str()?;

        group.assert_empty()?;
        meta.cmd = Some(cmd);
    } else if inner.next_if_ident("since").is_some() {
        // #[since = "..."]
        inner.expect(Token::Assignment(Assignment::Equal))?;
        meta.since = Some(inner.expect_str()?);
    } else if inner.next_if_ident("name").is_some() {
        // #[name = "..."]
        inner.expect(Token::Assignment(Assignment::Equal))?;
        meta.name = Some(inner.expect_str()?);
    } else if inner.next_if_ident("this").is_some() {
        // #[this]
        meta.this = true;
    } else if inner.next_if_ident("raw_json").is_some() {
        // #[raw_json]
        meta.raw_json = true;
    } else if inner.next_if_ident("enforce").is_some() {
        // #[enforce(...)]
        let mut group = inner.expect_group(BraceType::Parens)?;
        let mut enforce = Vec::new();

        if group.next_if_ident("clone").is_some() {
            enforce.push(EnforceType::Clone);
        }

        group.assert_empty()?;
        meta.enforce = enforce;
    } else if inner.next_if_ident("repr").is_some() {
        // #[repr(...)]
        let mut group = inner.expect_group(BraceType::Parens)?;
        let mut repr = Repr::Default;

        if group.next_if_ident("default").is_some() {
            repr = Repr::Default;
        } else if group.next_if_ident("object").is_some() {
            repr = Repr::Object;
        } else if group.next_if_ident("array").is_some() {
            repr = Repr::Array;
        } else if group.next_if_ident("string").is_some() {
            repr = Repr::String;
        } else if group.next_if_ident("byte").is_some() {
            repr = Repr::Byte;
        }

        group.assert_empty()?;
        meta.repr = repr;
    } else if inner.next_if_ident("same_as").is_some() {
        let mut group = inner.expect_group(BraceType::Parens)?;
        let ty = parse_typeref(&mut group, cx)?;

        group.assert_empty()?;
        meta.same_as.push(ty);
    } else if inner.next_if_ident("limit").is_some() {
        let mut group = inner.expect_group(BraceType::Parens)?;
        let a = parse_literal(&mut group, cx)?;

        group.expect(Token::Punct(Punct::Comma))?;

        let b = parse_literal(&mut group, cx)?;

        group.assert_empty()?;
        meta.limit = Some((a, b));
    } else if inner.next_if_ident("hint_id").is_some() {
        let mut group = inner.expect_group(BraceType::Parens)?;
        let hint = group.expect_ident()?;

        group.expect(Token::Assignment(Assignment::Equal))?;

        let id = group.expect_str()?;

        group.assert_empty()?;
        meta.hint_id = Some((hint, id));
    }

    inner.assert_empty()?;
    meta.span = c.end_span();

    Ok(meta)
}

fn merge_def_meta<'a>(a: &mut DefMeta<'a>, b: DefMeta<'a>) -> Result<()> {
    if let Some(builtin) = b.builtin {
        if a.builtin.is_some() {
            return Err(Error::DuplicateMeta {
                kind: "builtin",
                span: b.span.into(),
            });
        }

        a.builtin = Some(builtin);
    }

    if b.inline {
        if a.inline {
            return Err(Error::DuplicateMeta {
                kind: "inline",
                span: b.span.into(),
            });
        }

        a.inline = true;
    }

    for item in b.allow {
        if a.allow.contains(&item) {
            return Err(Error::DuplicateMeta {
                kind: "allow",
                span: b.span.into(),
            });
        }

        a.allow.push(item);
    }

    if b.nullable {
        if a.nullable {
            return Err(Error::DuplicateMeta {
                kind: "nullable",
                span: b.span.into(),
            });
        }

        a.nullable = true;
    }

    if let Some(hint) = b.hint {
        if a.hint.is_some() {
            return Err(Error::DuplicateMeta {
                kind: "hint",
                span: b.span.into(),
            });
        }

        a.hint = Some(hint);
    }

    for item in b.restrict {
        if a.restrict.contains(&item) {
            return Err(Error::DuplicateMeta {
                kind: "restrict",
                span: b.span.into(),
            });
        }

        a.restrict.push(item);
    }

    for item in b.require {
        if a.require.contains(&item) {
            return Err(Error::DuplicateMeta {
                kind: "require",
                span: b.span.into(),
            });
        }

        a.require.push(item);
    }

    if let Some(dsl) = b.dsl {
        if a.dsl.is_some() {
            return Err(Error::DuplicateMeta {
                kind: "dsl",
                span: b.span.into(),
            });
        }

        a.dsl = Some(dsl);
    }

    if let Some(cmd) = b.cmd {
        if a.cmd.is_some() {
            return Err(Error::DuplicateMeta {
                kind: "cmd",
                span: b.span.into(),
            });
        }

        a.cmd = Some(cmd);
    }

    if let Some(since) = b.since {
        if a.since.is_some() {
            return Err(Error::DuplicateMeta {
                kind: "since",
                span: b.span.into(),
            });
        }

        a.since = Some(since);
    }

    if let Some(name) = b.name {
        if a.name.is_some() {
            return Err(Error::DuplicateMeta {
                kind: "name",
                span: b.span.into(),
            });
        }

        a.name = Some(name);
    }

    if b.this {
        if a.this {
            return Err(Error::DuplicateMeta {
                kind: "this",
                span: b.span.into(),
            });
        }

        a.this = true;
    }

    for item in b.enforce {
        if a.enforce.contains(&item) {
            return Err(Error::DuplicateMeta {
                kind: "enforce",
                span: b.span.into(),
            });
        }

        a.enforce.push(item);
    }

    if b.repr != Repr::Default {
        if a.repr != Repr::Default {
            return Err(Error::DuplicateMeta {
                kind: "repr",
                span: b.span.into(),
            });
        }

        a.repr = b.repr;
    }

    a.same_as.extend(b.same_as);

    if let Some(limit) = b.limit {
        if a.limit.is_some() {
            return Err(Error::DuplicateMeta {
                kind: "limit",
                span: b.span.into(),
            });
        }

        a.limit = Some(limit);
    }

    if b.raw_json {
        if a.raw_json {
            return Err(Error::DuplicateMeta {
                kind: "raw_json",
                span: b.span.into(),
            });
        }

        a.raw_json = true;
    }

    if let Some(hint_id) = b.hint_id {
        if a.hint_id.is_some() {
            return Err(Error::DuplicateMeta {
                kind: "hint_id",
                span: b.span.into(),
            });
        }

        a.hint_id = Some(hint_id);
    }

    let min = (a.span, b.span).min_by(|it| it.start);
    let max = (a.span, b.span).max_by(|it| it.start);

    let span = min + max;

    a.span = span;

    Ok(())
}

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_def_meta<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<DefMeta<'a>> {
    let mut meta = DefMeta::default();

    while c.peek().is_some_and(|it| *it == Token::Punct(Punct::Hash)) {
        merge_def_meta(&mut meta, parse_single_def_meta(c, cx)?)?;
    }

    c.clear_peek();

    Ok(meta)
}
