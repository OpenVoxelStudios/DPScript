use crate::{
    dpscript::{
        ast::{cond::ConditionalNode, node::Node},
        lexer::{
            Result,
            parser::{BodyLexer, ValueLexer},
            util::{LexerMethods, check_one},
        },
        tokenizer::Token,
    },
    util::AddSpan,
};

impl BodyLexer {
    pub fn read_cond(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to read conditional...");

        let span = self.start_parse(Token::If)?;
        let (cond, _) = self.eat_until(Token::LeftBrace);
        let (body, end) = self.eat_block(Token::LeftBrace, Token::RightBrace);
        let span = span.add(end);

        let body = BodyLexer::new(self.namespace.clone(), body).parse()?;
        let cond = check_one(ValueLexer::new(self.namespace.clone(), cond).parse()?)?;

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
