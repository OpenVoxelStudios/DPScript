mod inner;
mod token;
mod misc;

pub use token::*;
pub use misc::*;

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
        while self.cursor.has_next() {
            self.tokenize_inner()?;
        }

        Ok(self)
    }

    pub fn tokens(&self) -> Vec<Spanned<Token>> {
        self.tokens.clone()
    }
}
