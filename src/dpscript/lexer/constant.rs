use crate::{
    dpscript::{
        ast::{constant::ConstantNode, node::Node},
        lexer::{
            Result,
            parser::ValueLexer,
            ty::TypeLexer,
            util::{LexerMethods, check_one},
        },
        tokenizer::Token,
    },
    util::AddSpan,
};

pub trait ConstLexer: LexerMethods + TypeLexer {
    fn read_const(&mut self) -> Result<Node> {
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

        let (value, last) = self.eat_until(Token::Semi);
        let span = span.add(last);
        let value = Box::new(check_one(ValueLexer::new(self.ns(), value).parse()?)?);

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

impl<T: LexerMethods + TypeLexer> ConstLexer for T {}
