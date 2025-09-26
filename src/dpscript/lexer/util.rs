use miette::{SourceOffset, SourceSpan};

use crate::{
    dpscript::{
        lexer::{Lexer, Result, err::LexerErr},
        tokenizer::Token,
    },
    util::Spanned,
};

impl Lexer {
    pub fn eat_id(&mut self) -> Result<String> {
        match self.eat() {
            Some((Token::Ident(id), _)) => Ok(id),

            Some((other, span)) => Err(LexerErr::ExpectedButGot {
                src: self.src(),
                span,
                expect: Token::Ident("".into()),
                got: other,
            }),

            None => Err(self.eof()),
        }
    }

    pub fn expect(&mut self, tkn: Token) -> Result<()> {
        match self.eat() {
            Some((token, span)) => {
                if token == tkn {
                    Ok(())
                } else {
                    Err(LexerErr::ExpectedButGot {
                        src: self.src(),
                        span,
                        expect: tkn,
                        got: token,
                    })
                }
            }

            None => Err(self.eof()),
        }
    }

    pub fn eat_until(&mut self, until: Token) -> Spanned<Vec<Spanned<Token>>> {
        let mut buf = Vec::new();
        let mut last = SourceSpan::new(SourceOffset::from_location("", 0, 0), 0);

        while let Some(tkn) = self.eat() {
            if tkn.0 == until {
                last = tkn.1;
                break;
            }

            buf.push(tkn);
        }

        (buf, last)
    }

    pub fn eat_block(&mut self, open: Token, close: Token) -> Spanned<Vec<Spanned<Token>>> {
        let mut buf = Vec::new();
        let mut opens = 1;
        let mut last = SourceSpan::new(SourceOffset::from_location("", 0, 0), 0);

        while let Some(tkn) = self.eat() {
            if tkn.0 == open {
                opens += 1;
            } else if tkn.0 == close {
                opens -= 1;
            }

            if opens <= 0 {
                last = tkn.1;
                break;
            }

            buf.push(tkn);
        }

        (buf, last)
    }

    pub fn start_parse(&mut self, token: Token) -> Result<SourceSpan> {
        match self.peek(0).cloned() {
            Some((tok, span)) => {
                if tok == token {
                    Ok(self.eat().unwrap().1)
                } else {
                    self.pop()?;

                    Err(LexerErr::StartParse {
                        src: self.src(),
                        span: span.clone(),
                        expect: token,
                        got: tok,
                    })
                }
            }

            None => Err(self.eof()),
        }
    }

    pub fn if_next_and_eat(&mut self, token: Token) -> bool {
        if self.peek(0).is_some_and(|it| it.0 == token) {
            self.eat().unwrap();

            true
        } else {
            false
        }
    }
}
