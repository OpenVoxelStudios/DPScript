use crate::{
    dpscript::{
        ast::{at::AtNode, node::Node},
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::Identifier,
};

impl Lexer {
    pub fn read_at(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to parse at block...", self.nesting);

        let mut span = self.start_parse(Token::At)?;

        let block_id = format!(
            "zzz/{}/funcs/{}/blocks/{}",
            self.module,
            self.func()?,
            self.block()?
        );

        let pos = Box::new(self.read_value()?);

        self.expect(Token::LeftBrace)?;
        self.inc_block()?;

        let mut body = Vec::new();

        while !self.if_next_and_eat_span(Token::RightBrace, &mut span) {
            body.push(self.read_body()?);
        }

        self.pop_in_place()?;

        debug!("[{}] Successfully read at block!", self.nesting);

        Ok(Node::At(AtNode {
            span,
            body,
            pos,
            scope: None,
            ident: Identifier {
                namespace: self.namespace.clone(),
                path: block_id.into(),
            },
        }))
    }
}
