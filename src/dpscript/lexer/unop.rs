use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            node::Node,
            unop::{UnaryOpNode, UnaryOperation},
        },
        lexer::{Result, parser::ValueLexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl ValueLexer {
    pub fn read_unop(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to parse unary...");

        let (tkn, span) = self.start_parse_any(vec![Token::Exclamation, Token::BitNot])?;

        let op = match tkn {
            Token::Exclamation => UnaryOperation::Negate,
            Token::BitNot => UnaryOperation::BitNot,
            _ => unreachable!(),
        };

        let rest = self.read_node()?;

        debug!("Successfully read unary!");

        self.pop_in_place()?;

        Ok(Node::UnaryOp(UnaryOpNode {
            operation: op,
            span: span.add(rest.span()),
            value: Box::new(rest),
        }))
    }
}
