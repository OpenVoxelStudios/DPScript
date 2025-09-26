use crate::dpscript::{
    ast::{
        literal::{LiteralData, LiteralNode},
        node::Node,
    },
    lexer::{Result, err::LexerErr, parser::ValueLexer, util::LexerMethods},
    tokenizer::Token,
};

impl ValueLexer {
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
}
