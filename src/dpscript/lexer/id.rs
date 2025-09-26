use crate::dpscript::{
    ast::{ident::IdentNode, node::Node},
    lexer::{Result, err::LexerErr, util::LexerMethods},
    tokenizer::Token,
};

pub trait IdentLexer: LexerMethods {
    fn read_ident(&mut self) -> Result<IdentNode> {
        let (ident, span) = self.eat_id()?;

        Ok(IdentNode { span, ident })
    }

    fn read_ident_full(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to parse ident...");

        match self.eat() {
            Some((Token::Ident(ident), span)) => {
                debug!("Successfully parsed ident!");

                self.pop_in_place()?;

                Ok(Node::Ident(IdentNode { span, ident }))
            }

            Some((other, span)) => {
                self.pop()?;

                Err(LexerErr::StartParse {
                    span,
                    expect: Token::Ident("".into()),
                    got: other,
                })
            }

            None => Err(self.eof()),
        }
    }
}

impl<T: LexerMethods> IdentLexer for T {}
