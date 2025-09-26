use crate::{
    dpscript::{
        ast::{
            literal::{LiteralData, LiteralNode},
            node::Node,
        },
        lexer::{Result, parser::ValueLexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl ValueLexer {
    pub fn read_array(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to read array...");

        let mut span = self.start_parse(Token::LeftBracket)?;

        let (content, content_span) = self.eat_block(Token::LeftBracket, Token::RightBracket);
        let content = ValueLexer::new(self.ns(), content).parse_sep(Token::Comma)?;

        span = span.add(content_span);

        self.pop_in_place()?;

        debug!("Successfully read array!");

        Ok(Node::Literal(LiteralNode {
            span,
            data: LiteralData::Array(content),
        }))
    }
}
