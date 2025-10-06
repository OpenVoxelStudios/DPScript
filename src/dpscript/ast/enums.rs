use std::fmt;

use super::ast::Scope;
use crate::dpscript::data::NodeInfo;
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct EnumNode {
    pub span: SourceSpan,

    /// The name of the enum.
    pub name: String,

    /// The enum's values.
    /// Each has their numerical ID assigned according to their order.
    /// This cannot be specified by the user currently.
    pub values: Vec<String>,
}

impl NodeInfo for EnumNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        // This is the declaration of an item, not a value. Therefore, it cannot be constant,
        // even if it gets removed during compilation and replaced with regular numbers.

        false
    }
}

impl fmt::Display for EnumNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "enum[{}]: [\n{}\n];", self.name, self.values.join(",\n"))
    }
}
