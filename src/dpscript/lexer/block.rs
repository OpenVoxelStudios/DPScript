use std::collections::BTreeMap;

use miette::SourceSpan;

use crate::{
    dpscript::{
        ast::{
            attr::AttrNode,
            block::{BlockKind, BlockNode},
            node::Node,
        },
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::Identifier,
};

impl Lexer {
    pub fn read_init_block(&mut self) -> Result<Node> {
        debug!("[{}] Attempting to read init block...", self.nesting);

        self.push();

        let attrs = self.read_attrs()?;
        let span = self.start_parse(Token::Init)?;

        self.read_block_inner(span, BlockKind::Init, attrs)
    }

    pub fn read_tick_block(&mut self) -> Result<Node> {
        debug!("[{}] Attempting to read tick block...", self.nesting);

        self.push();

        let attrs = self.read_attrs()?;
        let span = self.start_parse(Token::Tick)?;

        self.read_block_inner(span, BlockKind::Tick, attrs)
    }

    fn read_block_inner(
        &mut self,
        mut span: SourceSpan,
        kind: BlockKind,
        attrs: BTreeMap<String, AttrNode>,
    ) -> Result<Node> {
        self.expect(Token::LeftBrace)?;
        self.push_func(kind.to_string());

        let mut body = Vec::new();

        while !self.if_next_and_eat_span(Token::RightBrace, &mut span) {
            body.push(self.read_body()?);
        }

        self.pop_func()?;
        self.pop_in_place()?;

        debug!("[{}] Successfully read {kind} block!", self.nesting);

        self.event_block += 1;

        let id = attrs
            .get("name")
            .map(|it| {
                it.values
                    .first()
                    .map(|it| it.clone().as_literal().map(|it| it.as_string()))
            })
            .flatten()
            .flatten()
            .flatten()
            .unwrap_or(format!(
                "zzz/{}/blocks/{kind}/{}",
                self.module, self.event_block
            ));

        Ok(Node::Block(BlockNode {
            kind,
            span,
            body,
            attrs,
            keep: self.keep,
            ident: Identifier {
                namespace: self.namespace.clone(),
                path: id,
            },
        }))
    }
}
