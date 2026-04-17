use crate::data::{SourceSpan, Spanned};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ObjectiveNode<'a> {
    pub span: SourceSpan,

    /// The name of the objective (for reference in DPScript).
    pub name: Spanned<&'a str>,

    /// The actual ID of the scoreboard objective in Minecraft.
    pub id: &'a str,

    /// The objective criteria (/scoreboard objectives add [id] [criteria]).
    pub kind: Spanned<&'a str>,

    /// Whether this is publicly exported.
    pub is_public: bool,

    /// Whether to exclude this node from dead code elimination.
    pub keep: bool,
}

impl<'a> fmt::Display for ObjectiveNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keep = if self.keep { "[keep] " } else { "" };
        let public = if self.is_public { "[public] " } else { "" };

        write!(
            f,
            "{keep}objective {public}{} @ [{}] [kind: {}];",
            self.name.0, self.id, self.kind.0
        )
    }
}
