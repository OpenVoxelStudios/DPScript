use std::collections::HashMap;

use super::{Node, Subroutine, Type, Variable, attr::Attribute};
use crate::Spanned;
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Function {
    /// The function attributes.
    pub attrs: Vec<Attribute>,

    /// The function name.
    pub name: Spanned<String>,

    /// A list of arguments.
    pub args: Vec<FunctionArg>,

    /// The function return type.
    pub ret: Option<Type>,

    /// Is this a facade function? (A function that directly references a command)
    /// If this is true, `attrs` will have a `#[cmd = ...]` attribute.
    pub is_facade: bool,

    /// Is this a compiler builtin?
    pub is_compiler: bool,

    /// Is this function public?
    pub is_pub: bool,

    /// Is this function inline?
    pub is_inline: bool,

    /// The body of the function.
    pub body: Vec<Node>,

    /// The span.
    pub span: SourceSpan,

    /// A cache of local variables defined in the function.
    pub locals: Option<Vec<Variable>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionArg {
    pub attrs: Vec<Attribute>,
    pub name: Spanned<String>,
    pub ty: Type,
    pub span: SourceSpan,
    pub is_ref: bool,
}

impl Function {
    fn cache_vars(&mut self) -> &mut Self {
        let mut vars = Vec::new();

        for node in &self.body {
            if let Node::Variable(var) = node {
                vars.push(var.clone());
            }
        }

        self.locals = Some(vars);
        self
    }

    pub fn get_locals(&mut self) -> Vec<Variable> {
        if let Some(vars) = &self.locals {
            vars.clone()
        } else {
            self.cache_vars().locals.clone().unwrap_or_default()
        }
    }

    pub fn get_subroutines(&self) -> HashMap<String, Subroutine> {
        let mut map = HashMap::new();

        for item in &self.body {
            if let Node::Subroutine(sub) = item {
                map.insert(sub.name.clone().0, sub.clone());
            }
        }

        map
    }

    pub fn ir_name(&self, ns: impl AsRef<str>, module: impl AsRef<str>) -> String {
        format!(
            "{}:__dpscript_gen/{}/funcs/{}",
            ns.as_ref(),
            module.as_ref(),
            self.name.0
        )
    }
}
