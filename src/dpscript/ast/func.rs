use std::{collections::BTreeMap, fmt};

use crate::{
    dpscript::{
        ast::{
            ast::Scope,
            attr::AttrNode,
            node::Node,
            util::{Body, Indent},
        },
        data::NodeInfo,
        ty::TypeRef,
    },
    util::{DataLocation, Identifier},
};
use bitflags::bitflags;
use dpscript_macros::HasSpan;
use miette::SourceSpan;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    pub struct FuncFlags: u8 {
        const Inline   = 0b00000001;
        const Facade   = 0b00000010;
        const Compiler = 0b00000100;
        const Public   = 0b00001000;
        const Operator = 0b00010000;
    }
}

impl fmt::Display for FuncFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contains(FuncFlags::Inline) {
            write!(f, "    $flag: Inline;\n")?;
        }

        if self.contains(FuncFlags::Facade) {
            write!(f, "    $flag: Facade;\n")?;
        }

        if self.contains(FuncFlags::Compiler) {
            write!(f, "    $flag: Compiler;\n")?;
        }

        if self.contains(FuncFlags::Public) {
            write!(f, "    $flag: Public;\n")?;
        }

        if self.contains(FuncFlags::Operator) {
            write!(f, "    $flag: Operator;\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct FunctionNode {
    pub span: SourceSpan,
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub return_type: TypeRef,
    pub ident: Identifier,
    pub body: Vec<Node>,
    pub flags: FuncFlags,
    pub receiver: Option<String>,
    pub attrs: BTreeMap<String, AttrNode>,

    /// Whether to exclude this node from dead code elimination.
    pub keep: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct FunctionArg {
    pub span: SourceSpan,
    pub name: String,
    pub ty: TypeRef,
    pub location: DataLocation,
    pub is_this: bool,
    pub is_ref: bool,
    pub attrs: BTreeMap<String, AttrNode>,
}

impl NodeInfo for FunctionNode {
    // It's a function declaration and therefore has no value!
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }
}

impl fmt::Display for FunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let recv = match &self.receiver {
            Some(it) => format!("{it} -> "),
            None => "".into(),
        };

        let keep = if self.keep { "[keep] " } else { "" };

        write!(
            f,
            "{keep}fn[{recv}{}] @ [{}] -> {}:\n",
            self.name, self.ident, self.return_type,
        )?;

        self.flags.fmt(f)?;

        write!(
            f,
            "    $args: {{\n{}\n    }};\n    $body: {{\n{}\n    }};",
            self.args
                .iter()
                .map(|it| format!("{it}"))
                .collect::<Vec<_>>()
                .join("\n")
                .indent(8)
                .body(),
            self.body
                .iter()
                .map(|it| format!("{it}"))
                .collect::<Vec<_>>()
                .join("\n")
                .indent(8)
                .body()
        )?;

        Ok(())
    }
}

impl fmt::Display for FunctionArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ref_s = if self.is_ref { "[ref] " } else { "" };

        if self.is_this {
            write!(
                f,
                "$arg: (this) {ref_s}[{}] @ [{}]: [{}];",
                self.name, self.location, self.ty
            )
        } else {
            write!(
                f,
                "$arg: {ref_s}[{}] @ [{}]: [{}];",
                self.name, self.location, self.ty
            )
        }
    }
}
