mod owned;
mod tast;

pub use owned::*;
pub use tast::*;

use dpscript_core::Spanned;
use miette::Diagnostic;
use thiserror::Error;

pub use dpscript_tokenizer::tokenize_file as tokenize_first;

#[derive(Debug, Error, Diagnostic)]
pub enum FullError {
    #[error(transparent)]
    Tokenizer(#[from] dpscript_tokenizer::Error),

    #[error(transparent)]
    Tast(#[from] tast::Error),
}

pub fn tokenize_file<'a>(
    name: impl AsRef<str>,
    content: &'a str,
) -> Result<Vec<Spanned<Token<'a>>>, FullError> {
    let tokens = dpscript_tokenizer::tokenize_file(name, content)?;

    Ok(tast::tast_from_tokens(tokens)?)
}
