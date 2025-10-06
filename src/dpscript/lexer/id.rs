use crate::dpscript::{
    ast::{ident::IdentNode, node::Node},
    lexer::{parser::Lexer, util::LexerMethods, Result},
};

impl Lexer {
    pub fn read_ident(&mut self) -> Result<IdentNode> {
        let (ident, span) = self.eat_id()?;

        Ok(IdentNode { span, ident })
    }

    pub fn read_ident_full(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to parse ident...", self.nesting);

        let (ident, span) = self.start_parse_id()?;

        debug!("[{}] Successfully parsed ident!", self.nesting);

        self.pop_in_place()?;

        Ok(Node::Ident(IdentNode { span, ident }))
    }
}
