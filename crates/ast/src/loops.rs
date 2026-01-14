use std::fmt;

use crate::{
    data::{SourceSpan, Spanned},
    node::Node,
    util::{Body, Indent},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct LoopNode<'a> {
    pub span: SourceSpan,
    pub condition: LoopCondition<'a>,
    pub body: Vec<Node<'a>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub enum LoopCondition<'a> {
    Range {
        span: SourceSpan,
        /// The variable to put the current value in.
        var: Spanned<&'a str>,
        // Use i32 here because Minecraft scoreboards have the same min/max integer limits
        min: i32,
        max: i32,
    },

    Iter {
        span: SourceSpan,

        /// The variable to store the element.
        var: Spanned<&'a str>,

        /// The variable to loop through.
        array: Spanned<&'a str>,
    },

    While {
        /// Theoretically this span should always be the same as for
        /// [`LoopCondition::While::condition`], but it's here anyway
        /// because the [`LoopCondition::Range`] variant has it.
        span: SourceSpan,
        condition: Box<Node<'a>>,
    },
}

impl<'a> fmt::Display for LoopNode<'a> {
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

impl<'a> fmt::Display for LoopCondition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Range {
                span: _,
                var: (var, _),
                min,
                max,
            } => write!(f, "{var} = [{min} to {max}]"),

            Self::Iter {
                span: _,
                var: (var, _),
                array: (array, _),
            } => write!(f, "{var} = {array}"),

            Self::While { span: _, condition } => write!(f, "{condition}"),
        }
    }
}
