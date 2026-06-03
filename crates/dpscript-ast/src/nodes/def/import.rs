use crate::{prelude::{SourceSpan, def::DefTrait, meta::DefMeta}, util::Name};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct Import<'a> {
    pub paths: Vec<PathRef<'a>>,
    pub meta: DefMeta<'a>,

    /// The span the objective is defined in.
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct PathRef<'a> {
    pub parts: Vec<Name<'a>>,
    pub span: SourceSpan,
}

impl<'a> DefTrait<'a> for Import<'a> {
    fn with_meta(mut self, meta: DefMeta<'a>) -> Self {
        self.meta = meta;
        self
    }
}
