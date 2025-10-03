use crate::{
    dpscript::{
        ast::{call::CallNode, node::Node},
        lexer::{Result, err::LexerErr, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_call(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to parse call...");

        let (func, mut span) = match self.eat() {
            Some((Token::Ident(id), span)) => (id, span),

            Some((other, span)) => {
                self.pop()?;

                return Err(LexerErr::StartParse {
                    span,
                    expect: Token::Ident("".into()),
                    got: other,
                });
            }

            None => return Err(self.eof()),
        };

        self.start_parse(Token::LeftParen)?;

        let mut args = Vec::new();
        let mut first = true;

        while self.peek(0).is_some_and(|it| it.0 != Token::RightParen) {
            if first {
                first = false;
            } else {
                self.expect(Token::Comma)?;
            }

            args.push(self.read_value()?);
        }

        let last = self.expect_span(Token::RightParen)?;

        span = span.add(last);

        debug!("Successfully read call!");

        self.pop_in_place()?;

        Ok(Node::Call(CallNode {
            args,
            receiver: None, // TODO
            func,
            span,
        }))
    }
}
