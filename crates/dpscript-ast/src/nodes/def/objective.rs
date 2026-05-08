use crate::{
    prelude::{
        SourceSpan, Spanned,
        meta::{DefFlags, DefMeta},
    },
    util::Name,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Objective<'a> {
    /// The variable name of the objective.
    pub name: Name<'a>,

    /// The criteria of the objective.
    pub criteria: Spanned<&'a str>,

    /// The span the objective is defined in.
    pub span: SourceSpan,

    /// The definition's metadata.
    pub meta: DefMeta<'a>,

    /// The definition's flags.
    pub flags: Vec<DefFlags>,
}
