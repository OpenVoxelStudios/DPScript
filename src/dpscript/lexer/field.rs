use crate::{
    dpscript::{
        ast::{field::FieldNode, node::Node},
        lexer::{Result, parser::Lexer, ty::TypeLexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_field(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read field...", self.nesting);

        let is_public = self.if_next_and_eat(Token::Pub);
        let mut span = self.start_parse(Token::Field)?;
        let owner = self.read_ty()?;

        self.expect(Token::DoubleColon)?;

        let (name, _) = self.eat_id()?;

        self.expect(Token::Equal)?;

        let ty = self.read_ty()?;

        span = span.add(self.expect_span(Token::Semi)?);

        self.pop_in_place()?;

        debug!("[{}] Successfully read field!", self.nesting);

        Ok(Node::Field(FieldNode {
            is_public,
            name,
            owner,
            span,
            ty,
        }))
    }
}
