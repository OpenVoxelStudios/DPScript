use crate::{
    dpscript::{
        ast::{node::Node, var::VarNode},
        lexer::{
            Result,
            err::LexerErr,
            parser::{BodyLexer, ValueLexer},
            ty::TypeLexer,
            util::LexerMethods,
        },
        tokenizer::Token,
    },
    util::{AddSpan, DataLocation},
};

impl BodyLexer {
    pub fn read_var(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to read var...");

        let mut span = self.start_parse(Token::Let)?;
        let (name, _) = self.eat_id()?;
        let has_ty = self.if_next_and_eat(Token::Colon);
        let ty = if has_ty { Some(self.read_ty()?) } else { None };
        let has_val = self.if_next_and_eat(Token::Equal);
        let mut value = None;

        if has_val {
            let (tokens, body_span) = self.eat_until(Token::Semi);
            let mut read = ValueLexer::new(self.namespace.clone(), tokens).parse()?;

            if read.len() > 1 {
                return Err(LexerErr::MultipleValues { span: body_span });
            }
            
            self.backtrack(1);
            value = Some(Box::new(read.remove(0)));
        }

        let semi = self.expect_span(Token::Semi)?;

        span = span.add(semi);

        self.pop_in_place()?;

        debug!("Successfully read var!");

        Ok(Node::Variable(VarNode {
            name: name.clone(),
            span,
            ty,
            value,
            location: DataLocation {
                storage: "TODO".into(),
                path: name,
            },
        }))
    }
}
