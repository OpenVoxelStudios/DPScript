use crate::{
    dpscript::{
        ast::{
            literal::{LiteralData, LiteralNode},
            node::Node,
        },
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_array(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to read array...");

        let mut span = self.start_parse(Token::LeftBracket)?;
        let mut content = Vec::new();
        let mut first = true;

        while self.peek(0).is_some_and(|it| it.0 != Token::RightBracket) {
            if first {
                first = false;
            } else {
                self.expect(Token::Comma)?;
            }

            content.push(self.read_value()?);
        }

        let last = self.expect_span(Token::RightBracket)?;

        span = span.add(last);

        self.pop_in_place()?;

        debug!("Successfully read array!");

        Ok(Node::Literal(LiteralNode {
            span,
            data: LiteralData::Array(content),
        }))
    }
}
