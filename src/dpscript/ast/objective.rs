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

    pub is_public: bool,
}

impl NodeInfo for ObjectiveNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        // This is the declaration of a variable that gets removed during compilation, so therefore it is not constant.
        false
    }
}
