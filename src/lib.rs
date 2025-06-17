#[macro_use]
extern crate tracing;

pub mod cli;
pub mod compiler;
pub mod dpscript;
pub mod error;
pub mod lsp;
pub mod macros;
pub mod pack;
pub mod util;

pub type Result<T, E = crate::error::Error> = core::result::Result<T, E>;
