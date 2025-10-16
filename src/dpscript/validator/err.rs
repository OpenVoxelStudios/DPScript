use flexstr::SharedStr;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::dpscript::ty::TypeRef;

/// This is a code error on the user's end.
#[derive(Debug, Error, Diagnostic)]
pub enum ValidatorErr {
    #[error("Unresolved import: '{path}' (from module '{module}')")]
    #[diagnostic(code(validator::unresolved_import))]
    UnresolvedImport {
        #[label("here")]
        span: SourceSpan,
        path: SharedStr,
        module: SharedStr,
    },

    #[error("Duplicate import: '{name}")]
    #[diagnostic(code(validator::duplicate_import))]
    DuplicateImport {
        #[label("here")]
        span: SourceSpan,
        name: SharedStr,
    },

    #[error("Module not found: '{module}'")]
    #[diagnostic(code(validator::module_not_found))]
    ModuleNotFound {
        #[label("here")]
        span: SourceSpan,
        module: SharedStr,
    },

    #[error("Cannot compute type for value!")]
    #[diagnostic(code(validator::cannot_compute_type))]
    CannotComputeType {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Cannot index into a value that is not an array!")]
    #[diagnostic(code(validator::not_an_array))]
    NotAnArray {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Can only index an array with an integer!")]
    #[diagnostic(code(validator::non_integer_index))]
    NonIntIndex {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Unable to infer type for value!")]
    #[diagnostic(code(validator::cannot_infer_type))]
    CannotInferType {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Type mismatch: expected '{expected}', got '{got}'!")]
    #[diagnostic(code(validator::type_mismatch))]
    TypeMismatch {
        #[label("here")]
        span: SourceSpan,
        expected: TypeRef,
        got: TypeRef,
    },

    #[error("'{id}' is not a valid identifier!")]
    #[diagnostic(code(validator::invalid_ident))]
    InvalidIdent {
        #[label("here")]
        span: SourceSpan,
        id: SharedStr,
    },

    #[error("Unexpected function body!")]
    #[diagnostic(code(validator::unexpected_body))]
    UnexpectedBody {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Block must have a type!")]
    #[diagnostic(code(validator::untyped_block))]
    UntypedBlock {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Cannot perform binary operation on types '{lhs}' and '{rhs}'!")]
    #[diagnostic(code(validator::incompatible_types))]
    IncompatibleTypes {
        #[label("here")]
        span: SourceSpan,
        lhs: TypeRef,
        rhs: TypeRef,
    },

    #[error("Condition expression did not return a boolean!")]
    #[diagnostic(code(validator::cond_not_bool))]
    CondNotBool {
        #[label("here")]
        span: SourceSpan,
    },
}

/// An error occured during the validation step. This is ALWAYS a compiler bug.
#[derive(Debug, Error, Diagnostic)]
pub enum ValidationErr {
    #[error("Failed to process import - path was empty!")]
    #[diagnostic(code(compiler::empty_import_path))]
    EmptyImportPath {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("Scope stack was empty!.")]
    #[diagnostic(code(compiler::no_scope))]
    NoScope,
}

#[derive(Debug, Error, Diagnostic)]
#[diagnostic(severity(Warning))]
pub enum ValidatorWarn {
    #[error("Type inference on constants is discouraged.")]
    #[diagnostic(code(compiler::const_no_explicit_type))]
    ConstNoExplicitType {
        #[label("here")]
        span: SourceSpan,
    },

    /// *those who know.* \
    /// `This thing is my CAS project :P` \
    /// `- Redstone`
    #[error("PTSD.")]
    #[diagnostic(code(compiler::ib_ptsd))]
    IbPtsd {
        #[label("here")]
        span: SourceSpan,
    },
}

#[derive(Debug, Error, Diagnostic)]
#[error("Validation results:")]
#[diagnostic()]
pub struct AllErrors {
    #[source_code]
    pub code: NamedSource<SharedStr>,

    #[related]
    pub errors: Vec<ValidatorErr>,

    #[related]
    pub warnings: Vec<ValidatorWarn>,
}

pub type Err = ValidatorErr;
pub type Warn = ValidatorWarn;
pub type VErr = ValidationErr;

pub type Result<T, E = ValidationErr> = core::result::Result<T, E>;
