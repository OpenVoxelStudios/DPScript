#![feature(array_try_map)]

use peekmore::PeekMore;

use crate::token::{Token, TokenError, parse_next};

pub mod kw;
pub mod token;
pub mod util;

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
    let mut iter = input.chars().peekmore();
    let mut tokens = Vec::new();

    while let Some(token) = parse_next(&mut iter) {
        tokens.push(token?);
    }

    Ok(tokens)
}
