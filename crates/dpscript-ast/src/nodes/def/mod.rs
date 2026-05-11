use crate::prelude::{meta::DefMeta, types};

pub mod block;
pub mod constant;
pub mod enums;
pub mod func;
pub mod import;
pub mod objective;
pub mod structs;

crate::nodes::util::node_group! {
    Def = [
        constant::Constant,
        enums::Enum,
        func::Function,
        import::Import,
        objective::Objective,
        structs::Struct,
        block::Block,
        types::Typedef,
    ]
}

pub trait DefTrait<'a> {
    fn with_meta(self, meta: DefMeta<'a>) -> Self;
}

impl<'a> DefTrait<'a> for Def<'a> {
    fn with_meta(self, meta: DefMeta<'a>) -> Self {
        match self {
            Self::Constant(it) => Self::Constant(it.with_meta(meta)),
            Self::Enum(it) => Self::Enum(it.with_meta(meta)),
            Self::Function(it) => Self::Function(it.with_meta(meta)),
            Self::Import(it) => Self::Import(it.with_meta(meta)),
            Self::Objective(it) => Self::Objective(it.with_meta(meta)),
            Self::Struct(it) => Self::Struct(it.with_meta(meta)),
            Self::Block(it) => Self::Block(it.with_meta(meta)),
            Self::Typedef(it) => Self::Typedef(it.with_meta(meta)),
        }
    }
}
