use derivative::Derivative;

use crate::prelude::{SourceSpan, def::DefTrait, expr::Expr, meta::DefMeta, scope::Scope};

#[derive(Debug, Clone, Serialize, Facet, HasSpan, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct Block<'a> {
    /// The kind of block this is.
    pub kind: BlockKind,

    /// The block's body.
    pub body: Vec<Expr<'a>>,

    /// The block's span.
    pub span: SourceSpan,

    /// The definition's metadata.
    pub meta: DefMeta<'a>,

    #[facet(opaque)]
    #[derivative(PartialEq = "ignore")]
    pub scope: Option<Scope<'a>>,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet,
)]
pub enum BlockKind {
    Init,
    Tick,
}

impl<'a> DefTrait<'a> for Block<'a> {
    fn with_meta(mut self, meta: DefMeta<'a>) -> Self {
        self.meta = meta;
        self
    }
}
