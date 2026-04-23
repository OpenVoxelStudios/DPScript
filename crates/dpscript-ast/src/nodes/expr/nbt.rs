use crate::{data::SourceSpan, node::Node, util::serialize_snbt};
use std::{collections::BTreeMap, fmt};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct NbtValue<'a> {
    pub span: SourceSpan,
    pub data: NbtValueData<'a>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum NbtValueData<'a> {
    Map(BTreeMap<&'a str, NbtValue<'a>>),
    Array(Vec<NbtValue<'a>>),
    String(&'a str),
    Float(f32),
    Double(f64),
    Int(i32),
    Long(i64),
    Bool(bool),
    Byte(u8),

    /// An expression from the AST.
    Expr(Box<Node<'a>>),
}

impl<'a> fmt::Display for NbtValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "print-clarity")]
        {
            write!(f, "nbt<{}>", serialize_snbt(self, true))
        }

        #[cfg(not(feature = "print-clarity"))]
        {
            write!(f, "nbt<{}>", serialize_snbt(self, false))
        }
    }
}
