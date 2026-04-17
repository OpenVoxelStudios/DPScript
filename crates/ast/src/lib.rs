#[macro_use]
extern crate serde;

#[macro_use]
extern crate dpscript_macros;

pub mod ast;
pub mod at;
pub mod attr;
pub mod binop;
pub mod block;
pub mod call;
pub mod cond;
pub mod constant;
pub mod data;
pub mod enums;
pub mod field;
pub mod func;
pub mod import;
pub mod literal;
pub mod loc;
pub mod loops;
pub mod nbt;
pub mod node;
pub mod objective;
pub mod refs;
pub mod ret;
pub mod scope;
pub mod special;
pub mod var_info;
pub mod unop;
pub mod util;
pub mod var;

pub mod common {
    pub use crate::data::SourceSpan;
    pub use crate::data::Spanned;

    pub mod traits {
        pub use crate::data::HasSpan;
    }
}
