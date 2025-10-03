use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            literal::{LiteralData, LiteralNode},
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

        debug!("Attempting to read literal...");

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

        debug!("Attempting to read special...");

        let Some(cur) = self.eat() else {
            return Err(self.eof());
        };

        self.start_parse(Token::Colon)?;

        match cur {
            (Token::Selector, span) => {
                let id = self.eat_str()?;

                self.pop_in_place()?;

                debug!("Successfully read special!");

                Ok(Node::Special(SpecialNode {
                    data: SpecialData::Selector(id.0),
                    span: span.add(id.1),
                }))
            }

            (Token::Pos, span) => {
                self.expect(Token::LeftBracket)?;

                let x = self.read_value()?;

                self.expect(Token::Comma)?;

                let y = self.read_value()?;

                self.expect(Token::Comma)?;

                let z = self.read_value()?;

                self.expect(Token::RightBracket)?;

                self.pop_in_place()?;

                debug!("Successfully read special!");

                Ok(Node::Special(SpecialNode {
                    span: span.add(z.span()),
                    data: SpecialData::Pos(Box::new(x), Box::new(y), Box::new(z)),
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
