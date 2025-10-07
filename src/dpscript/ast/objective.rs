use std::fmt;

use miette::SourceSpan;

use crate::dpscript::data::NodeInfo;

use super::ast::Scope;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ObjectiveNode {
    pub span: SourceSpan,

    /// The name of the objective (for reference in DPScript).
    pub name: String,

    /// The actual ID of the scoreboard objective in Minecraft.
    pub id: String,

    /// The objective trigger (/scoreboard objectives add [id] [trigger]).
    pub kind: String,

    /// Whether this is publicly exported.
    pub is_public: bool,

    /// Whether to exclude this node from dead code elimination.
    pub keep: bool,
}

impl NodeInfo for ObjectiveNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        // This is the declaration of a variable that gets removed during compilation, so therefore it is not constant.
        false
    }
}

impl fmt::Display for ObjectiveNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keep = if self.keep { "[keep] " } else { "" };
        let public = if self.is_public { "[public] " } else { "" };

        write!(
            f,
            "{keep}objective {public}{} @ [{}] [kind: {}];",
            self.name, self.id, self.kind
        )
    }
}
