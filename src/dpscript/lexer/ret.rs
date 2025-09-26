use crate::{
    dpscript::{
        ast::{node::Node, ret::ReturnNode},
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
    pub fn read_return(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to read return...");

        let mut span = self.start_parse(Token::Return)?;
        let mut value = None;

        if !self.if_next_and_eat(Token::Semi) {
            let (rest, end) = self.eat_until(Token::Semi);

            span = span.add(end);
            value = Some(Box::new(check_one(
                ValueLexer::new(self.namespace.clone(), rest).parse()?,
            )?))
        }

        debug!("Successfully read return!");

        self.pop_in_place()?;

        Ok(Node::Return(ReturnNode { span, value }))
    }
}
