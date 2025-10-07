use crate::{
    dpscript::{
        ast::{import::ImportNode, node::Node},
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_import(&mut self) -> Result<Node> {
        debug!("[{}] Attempting to read import...", self.nesting);

        self.push();

        let span = self.start_parse(Token::Import)?;

        // TODO: rust-style multi-imports (in a later release, probably)

        let mut path = Vec::new();

        path.push(self.eat_id()?.0);

        while self.if_next_and_eat(Token::DoubleColon) {
            path.push(self.eat_id()?.0);
        }

        let last = self.expect_span(Token::Semi)?;

        self.pop_in_place()?;

        debug!("[{}] Successfully read import!", self.nesting);

        Ok(Node::Import(ImportNode {
            span: span.add(last),
            imports: vec![path],
        }))
    }
}
