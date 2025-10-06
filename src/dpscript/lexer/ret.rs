use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{node::Node, ret::ReturnNode},
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_return(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read return...", self.nesting);

        let mut span = self.start_parse(Token::Return)?;
        let mut value = None;

        if !self.if_next_and_eat(Token::Semi) {
            self.nesting += 1;

            let val = self.read_value()?;

            self.nesting -= 1;

            span = span.add(val.span());
            value = Some(Box::new(val));
            self.expect(Token::Semi)?;
        }

        debug!("[{}] Successfully read return!", self.nesting);

        self.pop_in_place()?;

        Ok(Node::Return(ReturnNode { span, value }))
    }
}
