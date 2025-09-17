use crate::{dpscript::{ast::node::Node, lexer::Lexer}, util::Spanned, Result};

impl Lexer {
    pub fn read_import(&mut self) -> Result<Spanned<Node>> {}
}
