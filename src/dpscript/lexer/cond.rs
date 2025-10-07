use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            cond::{ConditionalNode, ElseIfNode},
            node::Node,
        },
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
};

impl Lexer {
    pub fn read_cond(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read conditional...", self.nesting);

        let mut span = self.start_parse(Token::If)?;

        self.nesting += 1;

        let cond = self.read_value()?;

        self.nesting -= 1;

        self.expect(Token::LeftBrace)?;
        self.inc_block()?;

        let mut body = Vec::new();

        while !self.if_next_and_eat_span(Token::RightBrace, &mut span) {
            body.push(self.read_body()?);
        }

        let mut else_ifs = Vec::new();
        let mut else_body = Vec::new();

        while self.if_next_and_eat(Token::Else) {
            if self.if_next_and_eat(Token::If) {
                self.nesting += 1;

                let cond = self.read_value()?;

                self.nesting -= 1;

                self.expect(Token::LeftBrace)?;
                self.inc_block()?;

                let mut body = Vec::new();
                let mut span = cond.span();

                while !self.if_next_and_eat_span(Token::RightBrace, &mut span) {
                    body.push(self.read_body()?);
                }

                else_ifs.push(ElseIfNode {
                    span,
                    condition: cond,
                    body: body,
                })
            } else {
                self.expect(Token::LeftBrace)?;

                while !self.if_next_and_eat(Token::RightBrace) {
                    else_body.push(self.read_body()?);
                }

                break;
            }
        }

        debug!("[{}] Successfully read conditional!", self.nesting);

        self.pop_in_place()?;

        Ok(Node::Conditional(ConditionalNode {
            span,
            else_body,
            else_ifs,
            condition: Box::new(cond),
            body,
        }))
    }
}
