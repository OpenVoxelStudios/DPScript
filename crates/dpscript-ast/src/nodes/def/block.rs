use crate::prelude::{SourceSpan, expr::Expr, meta::DefMeta};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Block<'a> {
    /// The kind of block this is.
    pub kind: BlockKind,

    /// The block's body.
    pub body: Vec<Expr<'a>>,

    /// The block's span.
    pub span: SourceSpan,

    /// The definition's metadata.
    pub meta: DefMeta<'a>,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet,
)]
pub enum BlockKind {
    Init,
    Tick,
}
