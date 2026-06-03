use dpscript_core::MSourceSpan;
use miette::Diagnostic;
use thiserror::Error;

use crate::cx::AnalysisCx;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("incompatible types: {lhs} cannot be assigned from {rhs}")]
    IncompatibleTypes {
        /// lhs type
        lhs: String,

        /// rhs type
        rhs: String,

        #[label("here")]
        at: MSourceSpan,
    },

    #[error("cannot find module: {module}")]
    CannotFindModule {
        module: String,

        #[label("here")]
        at: MSourceSpan,
    },

    #[error("duplicate exports in module: {name}")]
    DuplicateExport {
        name: String,

        #[label("originally exported here:")]
        first: MSourceSpan,

        #[label("also exported here:")]
        new: MSourceSpan,
    },
}

#[derive(Debug, Error, Diagnostic)]
pub enum Warning {}

impl<'a> AnalysisCx<'a> {
    pub fn incompatible_types(
        &mut self,
        a: impl AsRef<str>,
        b: impl AsRef<str>,
        at: impl Into<MSourceSpan>,
    ) {
        self.err(Error::IncompatibleTypes {
            lhs: a.as_ref().into(),
            rhs: b.as_ref().into(),
            at: at.into(),
        });
    }

    pub fn cannot_find_module(&mut self, module: impl AsRef<str>, at: impl Into<MSourceSpan>) {
        self.err(Error::CannotFindModule {
            module: module.as_ref().into(),
            at: at.into(),
        });
    }

    pub fn duplicate_export(
        &mut self,
        name: impl AsRef<str>,
        first: impl Into<MSourceSpan>,
        new: impl Into<MSourceSpan>,
    ) {
        self.err(Error::DuplicateExport {
            name: name.as_ref().into(),
            first: first.into(),
            new: new.into(),
        });
    }
}
