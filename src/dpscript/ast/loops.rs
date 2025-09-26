use miette::SourceSpan;

use crate::dpscript::{ast::{ident::IdentNode, node::Node}, data::NodeInfo};

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
        /// The variable to put the current value in.
        var: IdentNode,
        // Use i32 here because Minecraft scoreboards have the same min/max integer limits
        min: i32,
        max: i32,
    },

    Iter {
        span: SourceSpan,

        /// The variable to store the element.
        var: IdentNode,

        /// The variable to loop through.
        array: IdentNode,
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
