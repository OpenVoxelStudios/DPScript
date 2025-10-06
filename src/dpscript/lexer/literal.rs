use std::collections::BTreeMap;

use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            literal::{LiteralData, LiteralNode},
            nbt::{NbtValue, NbtValueData},
            node::Node,
            special::{SpecialData, SpecialNode},
        },
        lexer::{Result, err::LexerErr, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_literal(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read literal...", self.nesting);

        match self.eat() {
            Some((Token::Int(val), span)) => {
                self.pop_in_place()?;

                Ok(Node::Literal(LiteralNode {
                    data: LiteralData::Int(val),
                    span,
                }))
            }

            Some((Token::Float(val), span)) => {
                self.pop_in_place()?;

                Ok(Node::Literal(LiteralNode {
                    data: LiteralData::Float(val),
                    span,
                }))
            }

            Some((Token::Double(val), span)) => {
                self.pop_in_place()?;

                Ok(Node::Literal(LiteralNode {
                    data: LiteralData::Double(val),
                    span,
                }))
            }

            Some((Token::Bool(val), span)) => {
                self.pop_in_place()?;

                Ok(Node::Literal(LiteralNode {
                    data: LiteralData::Bool(val),
                    span,
                }))
            }

            Some((Token::String(val), span)) => {
                self.pop_in_place()?;

                Ok(Node::Literal(LiteralNode {
                    data: LiteralData::String(val),
                    span,
                }))
            }

            Some((other, span)) => {
                self.pop()?;

                Err(LexerErr::ExpectedAnyToken {
                    span,
                    expected: vec![
                        Token::Int(0),
                        Token::Float(0.0),
                        Token::Double(0.0),
                        Token::Bool(false),
                        Token::String("".into()),
                    ],
                    got: other,
                })
            }

            None => Err(self.eof()),
        }
    }

    pub fn read_special(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read special...", self.nesting);

        let Some(cur) = self.eat() else {
            return Err(self.eof());
        };

        self.start_parse(Token::Colon)?;

        match cur {
            (Token::Selector, span) => {
                let id = self.eat_str()?;

                self.pop_in_place()?;

                debug!("[{}] Successfully read special!", self.nesting);

                Ok(Node::Special(SpecialNode {
                    data: SpecialData::Selector(id.0),
                    span: span.add(id.1),
                }))
            }

            (Token::Pos, span) => {
                self.expect(Token::LeftBracket)?;

                self.nesting += 1;

                let x = self.read_value()?;

                self.expect(Token::Comma)?;

                let y = self.read_value()?;

                self.expect(Token::Comma)?;

                let z = self.read_value()?;

                self.nesting -= 1;

                self.expect(Token::RightBracket)?;

                self.pop_in_place()?;

                debug!("[{}] Successfully read special!", self.nesting);

                Ok(Node::Special(SpecialNode {
                    span: span.add(z.span()),
                    data: SpecialData::Pos(Box::new(x), Box::new(y), Box::new(z)),
                }))
            }

            (Token::Component, span) | (Token::ComponentShort, span) => {
                // This is actually just syntactic sugar - it just turns it into NBT.
                // This still uses the special `Component` variant, though, since it can be validated
                // against the schema when validation of binary operations occurs.

                let (data, end) = self.eat_str()?;

                self.pop_in_place()?;

                debug!("[{}] Successfully read special!", self.nesting);

                let mut map = BTreeMap::new();

                map.insert(
                    "text".into(),
                    NbtValue {
                        span: end,
                        data: NbtValueData::String(data),
                    },
                );

                Ok(Node::Special(SpecialNode {
                    span: span.add(end),
                    data: SpecialData::Component(NbtValue {
                        span: end,
                        data: NbtValueData::Map(map),
                    }),
                }))
            }

            (other, span) => {
                self.pop()?;

                Err(LexerErr::ExpectedAnyToken {
                    span,
                    expected: vec![Token::Selector, Token::Pos],
                    got: other,
                })
            }
        }
    }
}
