mod compiler;
mod dep;
mod lowerer;

pub use compiler::*;
pub use dep::*;
pub use lowerer::*;

use miette::Diagnostic;
use thiserror::Error;

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
    Dependency(#[from] DependencyError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Ron(#[from] ron::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

impl From<miette::Report> for Error {
    fn from(value: miette::Report) -> Self {
        Error::Miette(value)
    }
}
