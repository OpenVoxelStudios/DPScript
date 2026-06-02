use core::fmt;

use dpscript_core::{MSourceSpan, bt::BtFrame};
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

    #[error("unexpected end of input")]
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

#[derive(Debug)]
pub struct WrappedError {
    pub inner: Error,
    pub backtrace: Vec<BtFrame>,
}

impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self.inner, f)?;

        if !self.backtrace.is_empty() {
            writeln!(f, "  Backtrace:")?;
        }

        for frame in &self.backtrace {
            writeln!(
                f,
                "    >> {}::{}\n       {}:{}",
                frame.module_path, frame.symbol, frame.file, frame.line
            )?;
        }

        Ok(())
    }
}

impl core::error::Error for WrappedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

impl Diagnostic for WrappedError {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.inner.code()
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        self.inner.diagnostic_source()
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.inner.help()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        self.inner.labels()
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        self.inner.related()
    }

    fn severity(&self) -> Option<miette::Severity> {
        self.inner.severity()
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        self.inner.source_code()
    }

    fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        self.inner.url()
    }
}
