use std::{collections::BTreeMap, fmt};

use miette::SourceSpan;

use crate::dpscript::ast::util::serialize_snbt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct NbtValue {
    pub span: SourceSpan,
    pub data: NbtValueData,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum NbtValueData {
    Map(BTreeMap<String, NbtValue>),
    Array(Vec<NbtValue>),
    String(String),
    Float(f32),
    Double(f64),
    Int(i32),
    Long(i64),
    Bool(bool),
    Byte(u8),
}

impl fmt::Display for NbtValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nbt<{}>", serialize_snbt(self, false))
    }
}
