use miette::SourceSpan;
use crate::dpscript::data::NodeInfo;
use super::ast::Scope;

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