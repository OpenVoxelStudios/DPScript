use std::collections::BTreeMap;

use crate::{
    dpscript::{
        ast::{
            literal::{LiteralData, LiteralNode},
            nbt::{NbtValue, NbtValueData},
            node::Node,
        },
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_nbt(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read NBT literal...", self.nesting);

        let span = self.start_parse(Token::Nbt)?;

        self.start_parse(Token::Colon)?;

        let start = self.expect_span(Token::LeftBrace)?;

        let (_tokens, end) = self.eat_block(Token::LeftBrace, Token::RightBrace);

        // TODO: Something with the tokens

        let span = span.add(end);

        self.pop_in_place()?;

        debug!("[{}] Successfully read NBT literal!", self.nesting);

        Ok(Node::Literal(LiteralNode {
            span,
            data: LiteralData::Nbt(NbtValue {
                span: start.add(end),
                data: NbtValueData::Map(BTreeMap::new()), // TODO,
            }),
        }))
    }
}
