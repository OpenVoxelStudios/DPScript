use std::collections::BTreeMap;

use crate::{
    common::traits::HasSpan,
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

        let value = self.read_nbt_value()?;
        let span = span.add(value.span());

        self.pop_in_place()?;

        debug!("[{}] Successfully read NBT literal!", self.nesting);

        Ok(Node::Literal(LiteralNode {
            span,
            data: LiteralData::Nbt(value),
        }))
    }

    fn read_nbt_value(&mut self) -> Result<NbtValue> {
        match self.peek(0).cloned() {
            // TODO: Long, short, byte
            Some((Token::Bool(v), _)) => {
                let (_, span) = self.eat().unwrap();

                Ok(NbtValue {
                    span,
                    data: NbtValueData::Bool(v),
                })
            }

            Some((Token::Int(v), _)) => {
                let (_, span) = self.eat().unwrap();

                Ok(NbtValue {
                    span,
                    // TODO: Long
                    data: NbtValueData::Int(v as i32),
                })
            }

            Some((Token::Float(v), _)) => {
                let (_, span) = self.eat().unwrap();

                Ok(NbtValue {
                    span,
                    data: NbtValueData::Float(v),
                })
            }

            Some((Token::Double(v), _)) => {
                let (_, span) = self.eat().unwrap();

                Ok(NbtValue {
                    span,
                    data: NbtValueData::Double(v),
                })
            }

            Some((Token::String(v), _)) => {
                let (_, span) = self.eat().unwrap();

                Ok(NbtValue {
                    span,
                    data: NbtValueData::String(v),
                })
            }

            Some((Token::LeftBracket, _)) => {
                let mut value = Vec::new();
                let start = self.expect_span(Token::LeftBracket)?;

                while self.peek(0).is_some_and(|it| it.0 != Token::RightBracket) {
                    value.push(self.read_nbt_value()?);

                    while self.if_next_and_eat(Token::Comma) {
                        // do nothing, it eats it :P
                    }
                }

                let end = self.expect_span(Token::RightBracket)?;

                Ok(NbtValue {
                    span: start.add(end),
                    data: NbtValueData::Array(value),
                })
            }

            Some((Token::LeftBrace, _)) => {
                let mut value = BTreeMap::new();
                let start = self.expect_span(Token::LeftBrace)?;

                while self.peek(0).is_some_and(|it| it.0 != Token::RightBrace) {
                    let (name, _) = self.eat_id()?;

                    self.expect(Token::Colon)?;
                    value.insert(name, self.read_nbt_value()?);

                    while self.if_next_and_eat(Token::Comma) {
                        // do nothing, it eats it :P
                    }
                }

                let end = self.expect_span(Token::RightBrace)?;

                Ok(NbtValue {
                    span: start.add(end),
                    data: NbtValueData::Map(value),
                })
            }

            Some(_) => {
                let data = self.read_value()?;

                Ok(NbtValue {
                    span: data.span(),
                    data: NbtValueData::Expr(Box::new(data)),
                })
            }

            None => Err(self.eof()),
        }
    }
}
