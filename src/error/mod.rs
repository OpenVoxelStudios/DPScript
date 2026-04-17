mod compiler;
mod dep;
mod lowerer;

pub use compiler::*;
pub use dep::*;
pub use lowerer::*;

use miette::Diagnostic;
use thiserror::Error;

use crate::dpscript::validator::err::{AllErrors, ValidationErr};

#[derive(Debug, Error, Diagnostic)]
#[error("Validation failed:")]
pub struct CompleteValidationErrors {
    #[related]
    pub errors: Vec<AllErrors>,
}

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("An error occured!")]
    #[diagnostic(code(dpscript::error::basic), url(docsrs))]
    Basic(#[help] String),

    #[error("An error occured!")]
    #[diagnostic(transparent)]
    Miette(miette::Report),

    #[error(transparent)]
    #[diagnostic(transparent)]
    AllValidator(#[from] AllErrors),

    #[error(transparent)]
    #[diagnostic(transparent)]
    AllValidator2(#[from] CompleteValidationErrors),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Validator(#[from] ValidationErr),

    // #[error(transparent)]
    // #[diagnostic(transparent)]
    // Compiler(#[from] CompilerError),

    // #[error(transparent)]
    // #[diagnostic(transparent)]
    // UnnamedCompiler(#[from] UnnamedCompilerError),

    // #[error(transparent)]
    // #[diagnostic(transparent)]
    // UnsourcedCompiler(#[from] UnsourcedCompilerError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Dependency(#[from] DependencyError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Ron(#[from] ron::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
    // #[error(transparent)]
    // Json5(#[from] json5::Error),

    // #[error(transparent)]
    // Json(#[from] serde_json::Error),
}

impl From<miette::Report> for Error {
    fn from(value: miette::Report) -> Self {
        Error::Miette(value)
    }
}
