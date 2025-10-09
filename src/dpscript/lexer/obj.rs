use crate::{
    dpscript::{
        ast::{node::Node, objective::ObjectiveNode},
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_objective(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to read objective...", self.nesting);

        let is_public = self.if_next_and_eat(Token::Pub);
        let span = self.start_parse(Token::Objective)?;
        let (name, _) = self.eat_id()?;

        self.expect(Token::Colon)?;

        let (ty, _) = self.eat_id()?;

        self.expect(Token::Equal)?;

        let (id, id_span) = self.eat_str()?;
        let span = span.add(id_span);

        self.pop_in_place()?;

        debug!("[{}] Successfully read objective...", self.nesting);

        Ok(Node::Objective(ObjectiveNode {
            id,
            kind: ty,
            name,
            span,
            is_public,
            keep: self.keep,
        }))
    }
}
