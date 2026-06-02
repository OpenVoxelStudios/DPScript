use crate::prelude::{
    SourceSpan,
    def::{DefTrait, import::PathRef},
    meta::DefMeta,
};

/// Exports all items from a module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Export<'a> {
    pub paths: Vec<PathRef<'a>>,
    pub meta: DefMeta<'a>,

    /// The span the objective is defined in.
    pub span: SourceSpan,
}

impl<'a> DefTrait<'a> for Export<'a> {
    fn with_meta(mut self, meta: DefMeta<'a>) -> Self {
        self.meta = meta;
        self
    }
}
