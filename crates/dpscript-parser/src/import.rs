use crate::{ParseCx, Rule, parse_err, util::next_or_die};
use ast::{
    data::{SourceSpan, Spanned},
    import::ImportNode,
    node::Node,
};
use miette::Result;
use pest::iterators::{Pair, Pairs};

pub struct ImportParser<'a> {
    pub pairs: Pairs<'a, Rule>,
    pub span: SourceSpan,
}

impl<'a> ImportParser<'a> {
    pub fn parse(mut self, cx: &mut ParseCx<'a>) -> Result<Vec<Node<'a>>> {
        let mut path = Vec::new();
        let mut res = Vec::new();

        while !self.pairs.is_empty() {
            let pair = next_or_die(cx, &mut self.pairs)?;

            res.extend(self.parse_item(cx, pair, &mut path)?);
        }

        let node = ImportNode {
            span: self.span,
            imports: res,
        };

        Ok(vec![Node::Import(node)])
    }

    fn parse_item(
        &mut self,
        cx: &mut ParseCx<'a>,
        pair: Pair<'a, Rule>,
        path: &mut Vec<&'a str>,
    ) -> Result<Vec<Spanned<Vec<&'a str>>>> {
        let mut res = Vec::new();

        let txt = pair.as_str();
        let rule = pair.as_rule();
        let span = pair.as_span();
        let mut inner = pair.into_inner();

        if inner.is_empty() {
            if rule == Rule::_ident {
                let mut item = path.clone();

                item.push(txt);
                res.push((item, span.into()));
            }
        } else {
            match rule {
                Rule::_ident => path.push(txt),

                Rule::_import_path => {
                    if inner.len() == 1 {
                        let next = next_or_die(cx, &mut inner)?;
                        let txt = next.as_str();

                        if next.as_rule() != Rule::_ident {
                            parse_err!(cx, "Path was not an identifier!");
                        }

                        let mut item = path.clone();

                        item.push(txt);
                        res.push((item, span.into()));
                    } else if inner.len() != 2 {
                        parse_err!(cx, "Invalid child length for import path: {:?}", inner);
                    } else {
                        let mut copy = path.clone();
                        let next = next_or_die(cx, &mut inner)?;

                        if next.as_rule() != Rule::_ident {
                            parse_err!(cx, "Path was not an identifier!");
                        }

                        copy.push(next.as_str());

                        let pair = next_or_die(cx, &mut inner)?;

                        res.extend(self.parse_item(cx, pair, &mut copy)?);
                    }
                }

                Rule::_import_obj => {
                    let mut copy = path.clone();

                    while !inner.is_empty() {
                        let pair = next_or_die(cx, &mut inner)?;

                        res.extend(self.parse_item(cx, pair, &mut copy)?);
                    }
                }

                r => parse_err!(
                    cx,
                    "Cannot parse unknown node while parsing import item: {:?}",
                    r
                ),
            }
        }

        Ok(res)
    }
}
