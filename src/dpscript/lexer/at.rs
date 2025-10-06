use crate::{
    dpscript::{
        ast::{at::AtNode, node::Node},
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_at(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to parse at block...", self.nesting);

        let span = self.start_parse(Token::At)?;
        let pos = Box::new(self.read_value()?);

        self.expect(Token::LeftBrace)?;

        let (body, end) = self.eat_block(Token::LeftBrace, Token::RightBrace);
        let body = Lexer::new(self.namespace.clone(), body).parse_body()?;
        let span = span.add(end);

        self.pop_in_place()?;

        debug!("[{}] Successfully read at block!", self.nesting);

        Ok(Node::At(AtNode { span, body, pos }))
    }
}
