use crate::data::{SourceSpan, Spanned};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct ImportNode<'a> {
    pub span: SourceSpan,

    /// A list of paths to imports (each part separated by '::' is a different element in the sub-Vec).
    // TODO: A better format for this
    pub imports: Vec<Spanned<Vec<&'a str>>>,
}

impl<'a> fmt::Display for ImportNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for set in &self.imports {
            write!(f, "import [{}];", set.0.join(", "))?;
        }

        Ok(())
    }
}
