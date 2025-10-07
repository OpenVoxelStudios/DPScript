use std::fmt;

use dpscript_macros::HasSpan;
use miette::SourceSpan;

use crate::dpscript::{
    ast::{
        node::Node,
        util::{Body, Indent},
    },
    data::NodeInfo,
    ty::TypeRef,
};

use super::ast::Scope;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ConditionalNode {
    pub span: SourceSpan,

    /// The condition for this 'if' node.
    pub condition: Box<Node>,

    /// The body of this 'if' block. If this is empty, it should be optimized
    /// out, but the conditions for 'else if' blocks and the 'else' block should
    /// still apply based on the root condition.
    pub body: Vec<Node>,

    /// Each 'else if' block will only run if all of the previous
    /// conditions are false. Therefore, this Vec must be kept in order.
    pub else_ifs: Vec<ElseIfNode>,

    /// The body of the 'else' block.
    /// This block will only run if *all* of:
    /// - The condition in the root returns false
    /// - The condition in every 'else if' node returns false
    /// If this is not defined by the user, this vec will be empty.
    /// If this block is empty, it should be optimized out.
    pub else_body: Vec<Node>,
}

/// This should never be used on its own, only as part of a [`ConditionalNode`],
/// so it won't be a valid variant for a regular [`Node`].
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct ElseIfNode {
    pub span: SourceSpan,
    pub condition: Node,
    pub body: Vec<Node>,
}

impl NodeInfo for ConditionalNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }

    fn returns(&self, _scope: &Scope) -> Option<TypeRef> {
        None
    }
}

impl NodeInfo for ElseIfNode {
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }

    fn returns(&self, _scope: &Scope) -> Option<TypeRef> {
        None
    }
}

impl fmt::Display for ConditionalNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "@if [{}]: {{\n{}\n}};\n",
            self.condition,
            self.body
                .iter()
                .map(|it| format!("{it}"))
                .collect::<Vec<_>>()
                .join("\n")
                .indent(4)
                .body()
        )?;

        for node in &self.else_ifs {
            write!(
                f,
                "@elif [{}]: {{\n{}\n}};\n",
                node.condition,
                node.body
                    .iter()
                    .map(|it| format!("{it}"))
                    .collect::<Vec<_>>()
                    .join("\n")
                    .indent(4)
                    .body()
            )?;
        }

        write!(
            f,
            "@else: {{\n{}\n}};",
            self.else_body
                .iter()
                .map(|it| format!("{it}"))
                .collect::<Vec<_>>()
                .join("\n")
                .indent(4)
                .body()
        )?;

        Ok(())
    }
}
