use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{constant::ConstantNode, node::Node},
        lexer::{Result, parser::Lexer, ty::TypeLexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_const(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to read constant...");

        let is_public = self.if_next_and_eat(Token::Pub);
        let span = self.start_parse(Token::Const)?;
        let (id, _) = self.eat_id()?;

        let ty = if self.if_next_and_eat(Token::Colon) {
            Some(self.read_ty()?)
        } else {
            None
        };

        self.expect(Token::Equal)?;

        let value = Box::new(self.read_value()?);
        let span = span.add(value.span());

        self.pop_in_place()?;

        debug!("Successfully read constant!");

        Ok(Node::Constant(ConstantNode {
            is_public,
            name: id,
            span,
            ty,
            value,
        }))
    }
}
