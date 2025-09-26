use crate::{
    dpscript::{
        ast::{call::CallNode, node::Node},
        lexer::{
            Result,
            err::LexerErr,
            parser::{BodyLexer, ValueLexer},
            util::LexerMethods,
        },
        tokenizer::Token,
    },
    util::AddSpan,
};

pub trait CallLexer: LexerMethods {
    fn read_call(&mut self) -> Result<Node> {
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

        let (args, args_span) = self.eat_block(Token::LeftParen, Token::RightParen);
        let args = ValueLexer::new(self.ns(), args).parse_sep(Token::Comma)?;

        span = span.add(args_span);

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

impl CallLexer for BodyLexer {}
impl CallLexer for ValueLexer {}
