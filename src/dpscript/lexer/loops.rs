use crate::{
    dpscript::{
        ast::{
            loops::{LoopCondition, LoopNode},
            node::Node,
        },
        lexer::{Result, err::LexerErr, id::IdentLexer, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_for_loop(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to read loop...");

        let span = self.start_parse(Token::For)?;
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

        let (body, end) = self.eat_block(Token::LeftBrace, Token::RightBrace);
        let body = Lexer::new(self.namespace.clone(), body).parse_body()?;
        let span = span.add(end);

        self.pop_in_place()?;

        debug!("Successfully read for loop!");

        Ok(Node::Loop(LoopNode {
            body,
            condition,
            span,
        }))
    }

    // TODO: While loop
}
