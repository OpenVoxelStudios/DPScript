use miette::{SourceOffset, SourceSpan};

use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::node::Node,
        lexer::{Result, err::LexerErr},
        tokenizer::Token,
    },
    util::{AddSpan, Spanned},
};

pub trait LexerMethods {
    fn stack(&mut self) -> &mut Vec<usize>;
    fn pos(&self) -> usize;
    fn set_pos(&mut self, pos: usize);
    fn last_pos(&self) -> SourceSpan;
    fn set_last_pos(&mut self, pos: SourceSpan);
    fn tokens(&self) -> &Vec<Spanned<Token>>;
    fn ns(&self) -> String;

    fn push(&mut self) {
        let pos = self.pos();

        self.stack().push(pos);
    }

    fn pop(&mut self) -> Result<()> {
        let pos = self
            .stack()
            .pop()
            .ok_or(LexerErr::StackPop { span: self.loc() })?;

        self.set_pos(pos);

        Ok(())
    }

    /// The same as [`Self::pop`], but without resetting the position.
    fn pop_in_place(&mut self) -> Result<()> {
        self.stack()
            .pop()
            .ok_or(LexerErr::StackPop { span: self.loc() })?;

        Ok(())
    }

    fn eat(&mut self) -> Option<Spanned<Token>> {
        self.set_pos(self.pos() + 1);

        if let Some(tkn) = self.peek(0) {
            self.set_last_pos(tkn.1.clone());
        }

        self.tokens().get(self.pos() - 1).cloned()
    }

    fn has_next(&self) -> bool {
        self.pos() < self.tokens().len()
    }

    fn peek(&self, amount: usize) -> Option<&Spanned<Token>> {
        self.tokens().get(self.pos() + amount)
    }

    fn loc(&self) -> SourceSpan {
        self.last_pos().clone()
    }

    fn eof(&self) -> LexerErr {
        LexerErr::EOF { span: self.loc() }
    }

    fn eat_id(&mut self) -> Result<(String, SourceSpan)> {
        match self.eat() {
            Some((Token::Ident(id), span)) => Ok((id, span)),

            Some((other, span)) => Err(LexerErr::ExpectedButGot {
                span,
                expect: Token::Ident("".into()),
                got: other,
            }),

            None => Err(self.eof()),
        }
    }

    fn eat_str(&mut self) -> Result<(String, SourceSpan)> {
        match self.eat() {
            Some((Token::String(s), span)) => Ok((s, span)),

            Some((tkn, span)) => Err(LexerErr::ExpectedButGot {
                span,
                expect: Token::String("".into()),
                got: tkn,
            }),

            None => Err(self.eof()),
        }
    }

    fn eat_int(&mut self) -> Result<(i64, SourceSpan)> {
        match self.eat() {
            Some((Token::Int(i), span)) => Ok((i, span)),

            Some((tkn, span)) => Err(LexerErr::ExpectedButGot {
                span,
                expect: Token::Int(0),
                got: tkn,
            }),

            None => Err(self.eof()),
        }
    }

    fn eat_uint(&mut self) -> Result<(u64, SourceSpan)> {
        match self.eat() {
            Some((Token::Int(i), span)) => {
                if i < 0 {
                    Err(LexerErr::IntWasNegative { span })
                } else {
                    Ok((i as u64, span))
                }
            }

            Some((tkn, span)) => Err(LexerErr::ExpectedButGot {
                span,
                expect: Token::Int(0),
                got: tkn,
            }),

            None => Err(self.eof()),
        }
    }

    fn eat_int32(&mut self) -> Result<(i32, SourceSpan)> {
        match self.eat() {
            Some((Token::Int(i), span)) => {
                if i < i32::MIN as i64 || i > i32::MAX as i64 {
                    Err(LexerErr::IntMaxVal { span })
                } else {
                    Ok((i as i32, span))
                }
            }

            Some((tkn, span)) => Err(LexerErr::ExpectedButGot {
                span,
                expect: Token::Int(0),
                got: tkn,
            }),

            None => Err(self.eof()),
        }
    }

    fn expect(&mut self, tkn: Token) -> Result<()> {
        match self.eat() {
            Some((token, span)) => {
                if token == tkn {
                    Ok(())
                } else {
                    Err(LexerErr::ExpectedButGot {
                        span,
                        expect: tkn,
                        got: token,
                    })
                }
            }

            None => Err(self.eof()),
        }
    }

    fn expect_span(&mut self, tkn: Token) -> Result<SourceSpan> {
        match self.eat() {
            Some((token, span)) => {
                if token == tkn {
                    Ok(span)
                } else {
                    Err(LexerErr::ExpectedButGot {
                        span,
                        expect: tkn,
                        got: token,
                    })
                }
            }

            None => Err(self.eof()),
        }
    }

    fn eat_until(&mut self, until: Token) -> Spanned<Vec<Spanned<Token>>> {
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

    fn eat_block(&mut self, open: Token, close: Token) -> Spanned<Vec<Spanned<Token>>> {
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

    fn start_parse(&mut self, token: Token) -> Result<SourceSpan> {
        match self.peek(0).cloned() {
            Some((tok, span)) => {
                if tok == token {
                    Ok(self.eat().unwrap().1)
                } else {
                    self.pop()?;

                    Err(LexerErr::StartParse {
                        span: span.clone(),
                        expect: token,
                        got: tok,
                    })
                }
            }

            None => {
                self.pop()?;

                Err(LexerErr::StartParse {
                    span: self.loc(),
                    expect: token,
                    got: Token::EOF,
                })
            }
        }
    }

    fn start_parse_any(&mut self, tokens: Vec<Token>) -> Result<(Token, SourceSpan)> {
        match self.peek(0).cloned() {
            Some((tok, span)) => {
                if tokens.contains(&tok) {
                    Ok(self.eat().unwrap())
                } else {
                    self.pop()?;

                    Err(LexerErr::ExpectedAnyToken {
                        span: span.clone(),
                        expected: tokens,
                        got: tok,
                    })
                }
            }

            None => Err(self.eof()),
        }
    }

    fn if_next_and_eat(&mut self, token: Token) -> bool {
        if self.peek(0).is_some_and(|it| it.0 == token) {
            self.eat().unwrap();

            true
        } else {
            false
        }
    }

    fn backtrack(&mut self, count: usize) {
        self.set_pos(self.pos() - count);
    }
}

pub fn check_one(mut nodes: Vec<Node>) -> Result<Node> {
    if nodes.len() > 1 {
        Err(LexerErr::MultipleValues {
            span: nodes.remove(0).span().add(nodes.pop().unwrap().span()),
        })
    } else {
        Ok(nodes.remove(0))
    }
}

#[macro_export]
macro_rules! impl_lexer {
    ($it: ident) => {
        impl $crate::dpscript::lexer::util::LexerMethods for $it {
            fn stack(&mut self) -> &mut Vec<usize> {
                &mut self.stack
            }

            fn pos(&self) -> usize {
                self.pos
            }

            fn set_pos(&mut self, pos: usize) {
                self.pos = pos;
            }

            fn last_pos(&self) -> miette::SourceSpan {
                self.last_pos
            }

            fn set_last_pos(&mut self, pos: miette::SourceSpan) {
                self.last_pos = pos;
            }

            fn tokens(&self) -> &Vec<$crate::util::Spanned<$crate::dpscript::tokenizer::Token>> {
                &self.tokens
            }
            fn ns(&self) -> String {
                self.namespace.clone()
            }
        }
    };
}

#[macro_export]
macro_rules! chain_parsers {
    (($value: expr): $self: ident; [$($parser: ident),*]) => {
        let mut expected = Vec::new();
        let mut span = $self.loc();
        let mut got = Token::None;

        $(
            tracing::debug!("Tokens: {:#?}", $self.tokens);
            tracing::debug!("Pos: {}", $self.pos);

            let res = $self.$parser();

            match res {
                Ok(node) => {
                    if !$value {
                        $self.if_next_and_eat(Token::Semi);
                    }

                    return Ok(node);
                }

                Err(LexerErr::StartParse { span: sp, expect, got: tkn }) => {
                    expected.push(expect);

                    if got != tkn {
                        got = tkn;
                    }

                    if sp.len() > span.len() {
                        span = sp;
                    }
                }

                Err(LexerErr::ExpectedAnyToken { span: sp, expected: tokens, got: tkn }) => {
                    expected.extend(tokens);

                    if got != tkn {
                        got = tkn;
                    }

                    if sp.len() > span.len() {
                        span = sp;
                    }
                }

                Err(other) => return Err(other),
            };
        )*

        return Err(LexerErr::ExpectedAny {
            span,
            got,
            expected: expected
                .into_iter()
                .map(|it| format!("'{it:?}'"))
                .collect::<Vec<_>>().join(", ")
        });
    };

    ($sep: ident, ($value: expr): $self: ident; [$($parser: ident),*]) => {
        if $self.peek(0).is_some_and(|it| it.0 == *$sep) {
            $self.eat();
            return Ok(None);
        }

        let mut expected = Vec::new();
        let mut span = $self.loc();
        let mut got = Token::None;

        $(
            tracing::debug!("Tokens: {:#?}", $self.tokens);
            tracing::debug!("Pos: {}", $self.pos);

            let res = $self.$parser();

            match res {
                Ok(node) => {
                    if !$value {
                        $self.if_next_and_eat(Token::Semi);
                    }

                    return Ok(Some(node));
                }

                Err(LexerErr::StartParse { span: sp, expect, got: tkn }) => {
                    expected.push(expect);

                    if got != tkn {
                        got = tkn;
                    }

                    if sp.len() > span.len() {
                        span = sp;
                    }
                }

                Err(LexerErr::ExpectedAnyToken { span: sp, expected: tokens, got: tkn }) => {
                    expected.extend(tokens);

                    if got != tkn {
                        got = tkn;
                    }

                    if sp.len() > span.len() {
                        span = sp;
                    }
                }

                Err(other) => return Err(other),
            };
        )*

        return Err(LexerErr::ExpectedAny {
            span,
            got,
            expected: expected
                .into_iter()
                .map(|it| format!("'{it:?}'"))
                .collect::<Vec<_>>().join(", ")
        });
    };
}
