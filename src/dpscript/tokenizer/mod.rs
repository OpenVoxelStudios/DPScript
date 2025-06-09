mod inner;
mod token;

pub use token::*;

use crate::{Result, Spanned, util::Cursor};
use miette::NamedSource;

pub type StringCursor = Cursor<String, NamedSource<String>>;

#[derive(Debug, Clone)]
pub struct Tokenizer {
    pub tokens: Vec<Spanned<Token>>,
    pub cursor: StringCursor,
}

impl Tokenizer {
    pub fn new(file: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        Self {
            tokens: Vec::new(),
            cursor: StringCursor::new_from_code(file, data),
        }
    }

    pub fn run(&mut self) -> Result<&mut Self> {
        while let Some(ch) = self.cursor.next() {
            self.tokenize_inner(ch)?;
        }

        Ok(self)
    }

    pub fn tokens(&self) -> Vec<Spanned<Token>> {
        self.tokens.clone()
    }
}
