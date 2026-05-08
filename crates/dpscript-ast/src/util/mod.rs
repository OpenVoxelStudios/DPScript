mod body;
mod indent;
pub mod nbt;

pub use body::Body;
pub use indent::Indent;

use crate::prelude::Spanned;

pub type Name<'a> = Spanned<&'a str>;
pub type ModulePath<'a> = Vec<Name<'a>>;
