use crate::token::{Token, TokenError, parse_next};
use dpscript_core::{Spanned, StringCursor};
use miette::{Diagnostic, NamedSource};
use thiserror::Error;

pub mod kw;
pub mod token;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("an error occured")]
    #[diagnostic(code(dpscript_tokenizer::error))]
    Tokenizer {
        #[source_code]
        source_code: NamedSource<String>,

        #[related]
        related: Vec<TokenError>,
    },

    #[error("IO error occured")]
    #[diagnostic(code(dpscript_tokenizer::io))]
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),
}

pub fn tokenize_file<'a>(
    name: impl AsRef<str>,
    content: &'a str,
) -> Result<Vec<Spanned<Token<'a>>>, Error> {
    match tokenize(&content) {
        Ok(tokens) => Ok(tokens),
        Err(it) => Err(Error::Tokenizer {
            source_code: NamedSource::new(name.as_ref(), content.to_string()),
            related: vec![it],
        }),
    }
}

fn tokenize<'a>(input: &'a str) -> Result<Vec<Spanned<Token<'a>>>, TokenError> {
    let mut iter = StringCursor::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = parse_next(&mut iter) {
        if let Some(token) = token? {
            tokens.push(token);
        }
    }

    Ok(tokens)
}
