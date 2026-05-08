use crate::prelude::{SourceSpan, def::func::FunctionInfo, value::Value};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Call<'a> {
    pub target: Box<Value<'a>>,
    pub args: Vec<Value<'a>>,
    pub resolved: Option<ResolvedCall<'a>>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct ResolvedCall<'a> {
    /// The span of the call site.
    pub span: SourceSpan,

    /// The span the function was defined in.
    pub source_span: SourceSpan,

    /// The file the function was defined in.
    pub source_file: Option<PathBuf>,

    /// The target value (if any).
    /// If the call was `var.some_thing()`, this would be `var`.
    pub target: Option<Box<Value<'a>>>,

    /// The function we are calling.
    pub func: FunctionInfo<'a>,
}
