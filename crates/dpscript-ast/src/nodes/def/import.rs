use crate::{prelude::SourceSpan, util::Name};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Import<'a> {
    pub paths: Vec<PathRef<'a>>,

    /// The span the objective is defined in.
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct PathRef<'a> {
    pub parts: Vec<Name<'a>>,
    pub span: SourceSpan,
}
