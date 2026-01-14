use itertools::Itertools;
use std::fmt;

use crate::data::{SourceSpan, Spanned};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct EnumNode<'a> {
    pub span: SourceSpan,

    /// The name of the enum.
    pub name: Spanned<&'a str>,

    /// The enum's values.
    /// Each has their numerical ID assigned according to their order.
    pub values: Vec<Spanned<&'a str>>,
}

impl<'a> fmt::Display for EnumNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "enum[{}]: [\n{}\n];",
            self.name.0,
            self.values.iter().map(|it| it.0).collect_vec().join(",\n")
        )
    }
}
