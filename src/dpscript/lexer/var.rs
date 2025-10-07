use crate::{
    dpscript::{
        ast::{node::Node, var::VarNode},
        lexer::{Result, parser::Lexer, ty::TypeLexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::{AddSpan, DataLocation},
};

impl Lexer {
    pub fn read_var(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read var...", self.nesting);

        let mut span = self.start_parse(Token::Let)?;
        let (name, _) = self.eat_id()?;
        let has_ty = self.if_next_and_eat(Token::Colon);
        let ty = if has_ty { Some(self.read_ty()?) } else { None };
        let has_val = self.if_next_and_eat(Token::Equal);
        let mut value = None;

        if has_val {
            self.nesting += 1;
            value = Some(Box::new(self.read_value()?));
            self.nesting -= 1;
        }

        let semi = self.expect_span(Token::Semi)?;

        span = span.add(semi);

        self.pop_in_place()?;

        debug!("[{}] Successfully read var!", self.nesting);

        Ok(Node::Variable(VarNode {
            name: name.clone(),
            span,
            ty,
            value,
            location: DataLocation {
                storage: format!(
                    "{}:__dps/gen/funcs/{}/block/{}",
                    self.namespace,
                    self.func()?,
                    self.block()?
                ),
                path: name,
            },
        }))
    }
}
