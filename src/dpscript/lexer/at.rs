use crate::dpscript::{
    ast::{at::AtNode, node::Node},
    lexer::{Result, parser::Lexer, util::LexerMethods},
    tokenizer::Token,
};

impl Lexer {
    pub fn read_at(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to parse at block...", self.nesting);

        let mut span = self.start_parse(Token::At)?;
        let pos = Box::new(self.read_value()?);

        self.expect(Token::LeftBrace)?;
        self.inc_block()?;

        let mut body = Vec::new();

        while !self.if_next_and_eat_span(Token::RightBrace, &mut span) {
            body.push(self.read_body()?);
        }

        self.pop_in_place()?;

        debug!("[{}] Successfully read at block!", self.nesting);

        Ok(Node::At(AtNode {
            span,
            body,
            pos,
            scope: None,
        }))
    }
}
