use dpscript_core::SourceSpan;
use crate::prelude::value::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Assign<'a> {
    pub lhs: Box<Value<'a>>,
    pub op: AssignOp,
    pub rhs: Box<Value<'a>>,
    pub span: SourceSpan,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet,
)]
pub enum AssignOp {
    Eq,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
}
