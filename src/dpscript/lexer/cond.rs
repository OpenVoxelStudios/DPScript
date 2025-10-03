use crate::{
    dpscript::{
        ast::{cond::ConditionalNode, node::Node},
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_cond(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to read conditional...");

        let span = self.start_parse(Token::If)?;
        let cond = self.read_value()?;

        self.expect(Token::LeftBrace)?;

        let (body, end) = self.eat_block(Token::LeftBrace, Token::RightBrace);
        let span = span.add(end);
        let body = Lexer::new(self.namespace.clone(), body).parse_body()?;

        debug!("Successfully read conditional!");

        self.pop_in_place()?;

        Ok(Node::Conditional(ConditionalNode {
            span,
            else_body: vec![], // TODO
            else_ifs: vec![],  // TODO
            condition: Box::new(cond),
            body,
        }))
    }
}
