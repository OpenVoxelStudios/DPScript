use std::fmt;

use miette::SourceSpan;

use crate::dpscript::{
    ast::{
        ident::IdentNode,
        node::Node,
        util::{Body, Indent},
    },
    data::NodeInfo,
};

use super::ast::Scope;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct LoopNode {
    pub span: SourceSpan,
    pub condition: LoopCondition,
    pub body: Vec<Node>,
    pub scope: Option<Scope>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
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
    },
}

impl NodeInfo for LoopNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        // Loops cannot return values and therefore have no reason to be constant.
        false
    }
}

impl fmt::Display for LoopNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = match &self.condition {
            LoopCondition::Range {
                span: _,
                var: _,
                min: _,
                max: _,
            } => "range",

            LoopCondition::Iter {
                span: _,
                var: _,
                array: _,
            } => "iter",

            LoopCondition::While {
                span: _,
                condition: _,
            } => "while",
        };

        write!(
            f,
            "@loop<{ty}> [{}]: {{\n{}\n}};",
            self.condition,
            self.body
                .iter()
                .map(|it| format!("{it}"))
                .collect::<Vec<_>>()
                .join("\n")
                .indent(4)
                .body()
        )
    }
}

impl fmt::Display for LoopCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Range {
                span: _,
                var,
                min,
                max,
            } => write!(f, "{var} = [{min} to {max}]"),

            Self::Iter {
                span: _,
                var,
                array,
            } => write!(f, "{var} = {array}"),

            Self::While { span: _, condition } => write!(f, "{condition}"),
        }
    }
}
