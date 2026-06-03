use crate::cx::VisitCx;
use dpscript_core::MSourceSpan;
use miette::Diagnostic;
use thiserror::Error;

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

        cur_module: String,
    },

    #[error("cannot find module: {module}")]
    CannotFindModule {
        module: String,

        #[label("here")]
        at: MSourceSpan,

        cur_module: String,
    },

    #[error("duplicate exports in module: {name}")]
    DuplicateExport {
        name: String,

        #[label("originally exported here:")]
        first: MSourceSpan,

        #[label("also exported here:")]
        new: MSourceSpan,

        cur_module: String,
    },

    #[error("duplicate definitions: {name}")]
    DuplicateDefs {
        name: String,

        #[label("originally defined here:")]
        first: MSourceSpan,

        #[label("also defined here:")]
        new: MSourceSpan,

        cur_module: String,
    },

    #[error("unresolved import: {name}")]
    UnresolvedImport {
        name: String,

        #[label("here")]
        at: MSourceSpan,

        cur_module: String,
    },
}

#[derive(Debug, Error, Diagnostic)]
pub enum Warning {}

impl Error {
    pub fn module(&self) -> String {
        match self {
            Self::IncompatibleTypes { cur_module, .. } => cur_module.clone(),
            Self::CannotFindModule { cur_module, .. } => cur_module.clone(),
            Self::DuplicateExport { cur_module, .. } => cur_module.clone(),
            Self::DuplicateDefs { cur_module, .. } => cur_module.clone(),
            Self::UnresolvedImport { cur_module, .. } => cur_module.clone(),
        }
    }
}

impl<'a, 'visit> VisitCx<'a, 'visit> {
    pub fn incompatible_types(
        &mut self,
        a: impl AsRef<str>,
        b: impl AsRef<str>,
        at: impl Into<MSourceSpan>,
    ) {
        self.analysis.err(Error::IncompatibleTypes {
            lhs: a.as_ref().into(),
            rhs: b.as_ref().into(),
            at: at.into(),
            cur_module: self.module.name.clone(),
        });
    }

    pub fn cannot_find_module(&mut self, module: impl AsRef<str>, at: impl Into<MSourceSpan>) {
        self.analysis.err(Error::CannotFindModule {
            module: module.as_ref().into(),
            at: at.into(),
            cur_module: self.module.name.clone(),
        });
    }

    pub fn duplicate_export(
        &mut self,
        name: impl AsRef<str>,
        first: impl Into<MSourceSpan>,
        new: impl Into<MSourceSpan>,
    ) {
        self.analysis.err(Error::DuplicateExport {
            name: name.as_ref().into(),
            first: first.into(),
            new: new.into(),
            cur_module: self.module.name.clone(),
        });
    }

    pub fn duplicate_defs(
        &mut self,
        name: impl AsRef<str>,
        first: impl Into<MSourceSpan>,
        new: impl Into<MSourceSpan>,
    ) {
        self.analysis.err(Error::DuplicateDefs {
            name: name.as_ref().into(),
            first: first.into(),
            new: new.into(),
            cur_module: self.module.name.clone(),
        });
    }

    pub fn unresolved_import(&mut self, name: impl AsRef<str>, at: impl Into<MSourceSpan>) {
        self.analysis.err(Error::UnresolvedImport {
            name: name.as_ref().into(),
            at: at.into(),
            cur_module: self.module.name.clone(),
        });
    }
}
