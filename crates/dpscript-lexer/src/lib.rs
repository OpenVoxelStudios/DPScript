use crate::util::TokenCursor;
use dpscript_ast::prelude::Spanned;
use dpscript_parser::Token;

pub mod cx;
pub mod err;
pub mod parsers;
pub mod util;

pub type Result<'a, T, E = err::Error<'a>> = core::result::Result<T, E>;

pub fn parse(tokens: Vec<Spanned<Token>>) {
    let mut cursor = TokenCursor::new(tokens);
}
