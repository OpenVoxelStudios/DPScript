use crate::{
    dpscript::{
        ast::{call::CallNode, node::Node},
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_call(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to parse call...", self.nesting);

        let mut receiver = None;
        let (mut func, mut span) = self.start_parse_id()?;

        // TODO: Make this a vec
        if self.if_next_and_eat(Token::Dot) {
            receiver = Some(func);
            func = self.start_parse_id()?.0;
        }

        self.start_parse(Token::LeftParen)?;

        let mut args = Vec::new();

        while self.peek(0).is_some_and(|it| it.0 != Token::RightParen) {
            self.nesting += 1;

            args.push(self.read_value()?);

            self.nesting -= 1;

            if !self.peek(0).is_some_and(|it| it.0 == Token::RightParen) {
                self.expect(Token::Comma)?;
            }
        }

        let last = self.expect_span(Token::RightParen)?;

        span = span.add(last);

        debug!("[{}] Successfully read call!", self.nesting);

        self.pop_in_place()?;

        Ok(Node::Call(CallNode {
            args,
            receiver,
            func,
            span,
        }))
    }
}
