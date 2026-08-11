#[macro_use]
extern crate serde;

#[macro_use]
extern crate facet;

pub mod bt;

mod arr;
mod cursor;
mod ops;
mod span;
mod span_ops;

pub mod facet_impls;

pub use arr::*;
pub use cursor::*;
pub use ops::*;
pub use span::*;
pub use span_ops::*;

pub type Spanned<T> = (T, SourceSpan);
pub type MSourceSpan = miette::SourceSpan;

pub use dpscript_macros::*;
