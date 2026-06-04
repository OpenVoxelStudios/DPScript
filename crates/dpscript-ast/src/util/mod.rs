mod body;
mod indent;
pub mod nbt;

pub use body::Body;
use dpscript_core::SourceSpan;
pub use indent::Indent;

use crate::prelude::Spanned;

pub type Name<'a> = Spanned<&'a str>;
pub type ModulePath<'a> = Vec<Name<'a>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet)]
pub struct Remote<T> {
    pub module: String,
    pub span: SourceSpan,
    pub data: T,
}
