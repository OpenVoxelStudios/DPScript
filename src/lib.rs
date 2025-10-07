#![feature(error_generic_member_access, normalize_lexically)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate dpscript_macros;

pub mod cli;
pub mod common;
pub mod compiler;
pub mod dpscript;
pub mod error;
pub mod lsp;
pub mod macros;
pub mod pack;
pub mod util;

pub type Result<T, E = crate::error::Error> = core::result::Result<T, E>;
