use std::fmt;

use crate::{
    data::{SourceSpan, Spanned},
    node::Node,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct VarNode<'a> {
    pub span: SourceSpan,
    pub name: Spanned<&'a str>,
    pub ty: Option<Spanned<&'a str>>,
    pub value: Option<Box<Node<'a>>>,

    /// Is this variable a function argument?
    /// This shouldn't be set during lexing - only during validation (via [`super::func::FunctionArg::to_var`]).
    pub is_arg: bool,
}

impl<'a> fmt::Display for VarNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ty = if let Some((ty, _)) = &self.ty {
            format!(" [type: {ty}]")
        } else {
            "".into()
        };

        let val = if let Some(val) = &self.value {
            format!(" = {val}")
        } else {
            "".into()
        };

        write!(f, "var {}{ty}{val};", self.name.0)
    }
}
