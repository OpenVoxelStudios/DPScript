use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            node::Node,
            unop::{UnaryOpNode, UnaryOperation},
        },
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_unop(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to parse unary...", self.nesting);

        let (tkn, span) = self.start_parse_any(vec![
            Token::Exclamation,
            Token::Tilde,
            Token::Minus,
            Token::Range,
        ])?;

        let op = match tkn {
            Token::Exclamation => UnaryOperation::Invert,
            Token::Tilde => UnaryOperation::BitNot,
            Token::Minus => UnaryOperation::Negate,
            Token::Range => UnaryOperation::RangeEnd,

            _ => unreachable!(
                "How did this even happen? Please report this, this should NEVER happen."
            ),
        };

        self.nesting += 1;

        let rest = self.read_value()?;

        self.nesting -= 1;

        debug!("[{}] Successfully read unary!", self.nesting);

        self.pop_in_place()?;

        Ok(Node::UnaryOp(UnaryOpNode {
            op,
            span: span.add(rest.span()),
            value: Box::new(rest),
        }))
    }
}
