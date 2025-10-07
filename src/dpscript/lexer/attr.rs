use std::collections::BTreeMap;

use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::attr::AttrNode,
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    /// Read an attribute.
    /// THIS WILL NOT PUSH/POP THE STACK!
    pub fn read_attrs(&mut self) -> Result<BTreeMap<String, AttrNode>> {
        let mut attrs = BTreeMap::new();

        while self.if_next_and_eat(Token::Hash) && self.if_next_and_eat(Token::LeftBracket) {
            let (name, mut span) = self.eat_id()?;
            let mut values = Vec::new();

            if self.if_next_and_eat(Token::LeftParen) {
                while !self.if_next_and_eat_span(Token::RightParen, &mut span) {
                    values.push(self.read_value()?);

                    while self.if_next_and_eat(Token::Comma) {
                        // do nothing, it eats it :)
                    }
                }
            } else if self.if_next_and_eat(Token::Equal) {
                let value = self.read_value()?;

                span = span.add(value.span());
                values.push(value);
            }

            self.expect(Token::RightBracket)?;
            attrs.insert(name.clone(), AttrNode { span, name, values });
        }

        Ok(attrs)
    }
}
