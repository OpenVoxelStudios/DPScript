use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            loops::{LoopCondition, LoopNode},
            node::Node,
        },
        lexer::{Result, err::LexerErr, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_for_loop(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read loop...", self.nesting);

        let mut span = self.start_parse(Token::For)?;
        let var = self.read_ident()?;

        self.expect(Token::In)?;

        let condition = match self.peek(0) {
            Some((Token::Int(_), _)) => {
                let (min, _) = self.eat_int32()?;

                self.expect(Token::Range)?;

                let (max, end) = self.eat_int32()?;
                let cond_span = var.span.add(end);

                LoopCondition::Range {
                    span: cond_span,
                    min,
                    max,
                    var,
                }
            }

            Some((Token::Ident(_), _)) => {
                let array = self.read_ident()?;
                let cond_span = var.span.add(array.span);

                LoopCondition::Iter {
                    span: cond_span,
                    var,
                    array,
                }
            }

            Some((other, span)) => {
                return Err(LexerErr::Unexpected {
                    span: span.clone(),
                    tkn: other.clone(),
                });
            }

            None => return Err(self.eof()),
        };

        self.expect(Token::LeftBrace)?;
        self.inc_block()?;
        self.nesting += 1;

        let mut body = Vec::new();

        while !self.if_next_and_eat_span(Token::RightBrace, &mut span) {
            body.push(self.read_body()?);
        }

        self.nesting -= 1;
        self.pop_in_place()?;

        debug!("[{}] Successfully read for loop!", self.nesting);

        Ok(Node::Loop(LoopNode {
            body,
            condition,
            span,
        }))
    }

    pub fn read_while_loop(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read while loop...", self.nesting);

        let mut span = self.start_parse(Token::While)?;
        let cond = self.read_value()?;

        self.expect(Token::LeftBrace)?;
        self.inc_block()?;
        self.nesting += 1;

        let mut body = Vec::new();

        while !self.if_next_and_eat_span(Token::RightBrace, &mut span) {
            body.push(self.read_body()?);
        }

        self.nesting -= 1;
        self.pop_in_place()?;

        debug!("[{}] Successfully read while loop!", self.nesting);

        Ok(Node::Loop(LoopNode {
            body,
            condition: LoopCondition::While {
                span: cond.span(),
                condition: Box::new(cond),
            },
            span,
        }))
    }
}
