use crate::{ParseCx, inner::Rule, parse_err, parser::parse_next};
use ast::{
    data::{SpanUtil, Spanned},
    node::Node,
};
use miette::{NamedSource, Result, Severity, miette};
use pest::iterators::{Pair, Pairs};

pub trait ParserUtil<'a> {
    fn next_checked(&mut self, cx: &mut ParseCx<'a>) -> Option<Pair<'a, Rule>>;

    fn next_ident(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<&'a str>>;
    fn next_str(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<&'a str>>;
    fn next_as_str(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<&'a str>>;
    fn next_type(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<&'a str>>;
    fn next_expr(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<Pairs<'a, Rule>>>;
    fn check_next(&mut self, cx: &mut ParseCx<'a>, rule: Rule) -> Result<()>;
    fn one_inner(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<Pairs<'a, Rule>>>;

    fn parse_next(&mut self, cx: &mut ParseCx<'a>) -> Result<Vec<Node<'a>>>;
    fn parse_one_next(&mut self, cx: &mut ParseCx<'a>) -> Result<Node<'a>>;
}

impl<'a> ParserUtil<'a> for Pairs<'a, Rule> {
    fn next_checked(&mut self, cx: &mut ParseCx<'a>) -> Option<Pair<'a, Rule>> {
        let pair = self.next();

        if let Some(pair) = &pair {
            cx.pos = pair.as_span().into();
        }

        pair
    }

    fn next_ident(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<&'a str>> {
        let next = peek_or_die(cx, self)?;
        let span = next.as_span();
        let txt = next.as_str();

        if next.as_rule() != Rule::_ident {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::ident",
                labels = vec![span.label()],
                "Expected an identifier!"
            );
        }

        self.next_checked(cx).unwrap();

        Ok((txt, span.into()))
    }

    fn next_str(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<&'a str>> {
        let next = peek_or_die(cx, self)?;
        let rule = next.as_rule();
        let span = next.as_span();
        let mut inner = next.into_inner();

        if rule != Rule::_str {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::str",
                labels = vec![span.label()],
                "Expected a string!"
            );
        }

        self.next_checked(cx).unwrap();

        Ok((inner.next_as_str(cx)?.0, span.into()))
    }

    fn next_as_str(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<&'a str>> {
        let next = next_or_die(cx, self)?;
        let span = next.as_span();
        let txt = next.as_str();

        Ok((txt, span.into()))
    }

    fn next_type(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<&'a str>> {
        let next = peek_or_die(cx, self)?;
        let span = next.as_span();

        if next.as_rule() != Rule::_type {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::type",
                labels = vec![span.label()],
                "Expected a type!"
            );
        }

        self.next_checked(cx).unwrap();

        next.into_inner().next_ident(cx)
    }

    fn next_expr(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<Pairs<'a, Rule>>> {
        let next = peek_or_die(cx, self)?;
        let span = next.as_span();

        // An ident is technically an expr but sometimes the grammar has to
        // use it directly because of recursion

        if next.as_rule() == Rule::_ident {
            self.next_checked(cx).unwrap();

            return Ok((Pairs::single(next), span.into()));
        }

        if next.as_rule() != Rule::_expr {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "expected::expr",
                labels = vec![span.label()],
                "Expected an expr!"
            );
        }

        self.next_checked(cx).unwrap();

        Ok((next.into_inner(), span.into()))
    }

    fn check_next(&mut self, cx: &mut ParseCx<'a>, rule: Rule) -> Result<()> {
        let next = peek_or_die(cx, self)?;
        let span = next.as_span();

        if next.as_rule() != rule {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "generic::check_fail",
                labels = vec![span.label()],
                "Rule check failed! Unexpected: {next}"
            );
        }

        self.next_checked(cx).unwrap();

        Ok(())
    }

    fn one_inner(&mut self, cx: &mut ParseCx<'a>) -> Result<Spanned<Pairs<'a, Rule>>> {
        if self.len() != 1 {
            parse_err!(cx, "Expected only one pair, found {}!", self.len());
        }

        let next = next_or_die(cx, self)?;
        let span = next.as_span();

        Ok((next.into_inner(), span.into()))
    }

    fn parse_next(&mut self, cx: &mut ParseCx<'a>) -> Result<Vec<Node<'a>>> {
        let pair = next_or_die(cx, self)?;

        Ok(parse_next(cx, &mut Pairs::single(pair))?)
    }

    fn parse_one_next(&mut self, cx: &mut ParseCx<'a>) -> Result<Node<'a>> {
        let vec = self.parse_next(cx)?;

        only_one(cx, vec)
    }
}

pub fn next_or_die<'a>(
    cx: &mut ParseCx<'a>,
    pairs: &mut Pairs<'a, Rule>,
) -> Result<Pair<'a, Rule>> {
    pairs.next_checked(cx).ok_or(
        miette!(
            severity = Severity::Error,
            code = "eof",
            labels = vec![cx.pos.label()],
            "Unexpected end of input!"
        )
        .with_source_code(NamedSource::new(cx.file, cx.code.to_string())),
    )
}

pub fn peek_or_die<'a>(
    cx: &mut ParseCx<'a>,
    pairs: &mut Pairs<'a, Rule>,
) -> Result<Pair<'a, Rule>> {
    pairs.peek().ok_or(
        miette!(
            severity = Severity::Error,
            code = "eof",
            labels = vec![cx.pos.label()],
            "Unexpected end of input!"
        )
        .with_source_code(NamedSource::new(cx.file, cx.code.to_string())),
    )
}

pub fn only_one<'a, T>(cx: &mut ParseCx<'a>, mut vec: Vec<T>) -> Result<T> {
    if vec.len() != 1 {
        parse_err!(
            cx,
            "Expected only one node to be returned, got {}!",
            vec.len()
        );
    }

    Ok(vec.remove(0))
}
