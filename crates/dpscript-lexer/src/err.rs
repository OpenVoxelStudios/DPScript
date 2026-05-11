use dpscript_core::MSourceSpan;
use dpscript_parser::OwnedToken;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("unexpected token: {token}")]
    UnexpectedToken {
        token: OwnedToken,

        #[label("here")]
        span: MSourceSpan,
    },

    #[error("unexpected end of file")]
    Eof {
        #[label("here")]
        span: MSourceSpan,
    },

    #[error("duplicate meta definition for '{kind}'")]
    DuplicateMeta {
        kind: &'static str,

        #[label("here")]
        span: MSourceSpan,
    },

    #[error("missing operator for binary operation - this is a compiler bug!")]
    MissingOp {
        #[label("here")]
        span: MSourceSpan,
    },

    #[error(
        "skip this branch in attempting to parse. if you see this, report it! this should never be shown!"
    )]
    Skip,
}
