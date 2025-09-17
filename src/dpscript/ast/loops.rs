use miette::SourceSpan;

use crate::dpscript::{ast::node::Node, data::NodeInfo};

use super::ast::Scope;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct LoopNode {
    pub span: SourceSpan,
    pub condition: LoopCondition,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub enum LoopCondition {
    Range {
        span: SourceSpan,
        // Use i32 here because Minecraft scoreboards have the same min/max integer limits
        min: i32,
        max: i32,
    },

    While {
        /// Theoretically this span should always be the same as for
        /// [`LoopCondition::While::condition`], but it's here anyway
        /// because the [`LoopCondition::Range`] variant has it.
        span: SourceSpan,
        condition: Box<Node>,
    }
}

impl NodeInfo for LoopNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        // Loops cannot return values and therefore have no reason to be constant.
        false
    }
}
