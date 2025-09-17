pub mod import;

use crate::{dpscript::{ast::ast::AST, tokenizer::Token}, util::Spanned, Result};

pub struct Lexer {
    pub file_name: String,
    pub source: String,
    pub tokens: Vec<Spanned<Token>>,
}

impl Lexer {
    pub fn new(file_name: String, source: String, tokens: Vec<Spanned<Token>>) -> Self {
        Self {
            file_name,
            source,
            tokens,
        }
    }

    pub fn eat(&mut self) -> Spanned<Token> {
        self.tokens.remove(0)
    }

    pub fn has_next(&self) -> bool {
        !self.tokens.is_empty()
    }

    pub fn peek(&self, amount: usize) -> Option<&Spanned<Token>> {
        self.tokens.get(amount)
    }

    pub fn run(&self) -> Result<AST> {
        let mut ast = AST::new();

        Ok(ast)
    }
}
