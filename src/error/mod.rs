mod compiler;
mod dep;
mod lexer;
mod lowerer;
mod tokenizer;

pub use compiler::*;
pub use dep::*;
pub use lexer::*;
pub use lowerer::*;
pub use tokenizer::*;

use miette::Diagnostic;
use thiserror::Error;

use crate::dpscript::{
    lexer::err::{LexerErr, LexerFullErr},
    validator::err::{AllErrors, ValidationErr},
};

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("An error occured!")]
    #[diagnostic(code(dpscript::error::basic), url(docsrs))]
    Basic(#[help] String),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Tokenizer(#[from] TokenizerError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnnamedTokenizer(#[from] UnnamedTokenizerError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Lexer(#[from] LexerErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    FullLexer(#[from] LexerFullErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    AllValidator(#[from] AllErrors),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Validator(#[from] ValidationErr),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Compiler(#[from] CompilerError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnnamedCompiler(#[from] UnnamedCompilerError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    UnsourcedCompiler(#[from] UnsourcedCompilerError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Dependency(#[from] DependencyError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Ron(#[from] ron::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error(transparent)]
    Json5(#[from] json5::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
