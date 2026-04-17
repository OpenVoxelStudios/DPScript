use bitflags::bitflags;
use std::{cell::RefCell, collections::BTreeMap, fmt, rc::Rc};

use crate::{
    attr::AttrNode, data::{SourceSpan, Spanned}, loc::{DataLocation, Identifier}, node::Node, scope::Scope, util::{Body, Indent}
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    pub struct FuncFlags: u8 {
        const Inline   = 0b00000001;
        const Facade   = 0b00000010;
        const Compiler = 0b00000100;
        const Public   = 0b00001000;
        const Operator = 0b00010000;
        const Instance = 0b00100000;
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

        if self.contains(FuncFlags::Instance) {
            write!(f, "    $flag: Instance;\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct FunctionNode<'a> {
    pub span: SourceSpan,
    pub name: Spanned<&'a str>,
    pub args: Vec<FunctionArg<'a>>,
    pub return_type: Option<Spanned<&'a str>>,
    pub body: Vec<Node<'a>>,
    pub flags: FuncFlags,
    pub receiver: Option<Spanned<&'a str>>,
    pub attrs: BTreeMap<&'a str, AttrNode<'a>>,
    pub ident: Identifier<'a>,

    #[serde(skip)]
    pub scope: Option<Rc<RefCell<Scope<'a>>>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct FunctionArg<'a> {
    pub span: SourceSpan,
    pub name: Spanned<&'a str>,
    pub ty: Spanned<&'a str>,
    pub location: DataLocation<'a>,
    pub is_this: bool,
    pub is_ref: bool,
    pub attrs: BTreeMap<Spanned<&'a str>, AttrNode<'a>>,
}

impl<'a> fmt::Display for FunctionNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let recv = match &self.receiver {
            Some((it, _)) => format!("{it} -> "),
            None => "".into(),
        };

        write!(
            f,
            "fn[{recv}]: {} -> {}:\n",
            self.name.0,
            self.return_type
                .clone()
                .map(|it| it.0)
                .unwrap_or("<none>".into()),
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

impl<'a> fmt::Display for FunctionArg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ref_s = if self.is_ref { "[ref] " } else { "" };

        if self.is_this {
            write!(f, "$arg: (this) {ref_s}[{}]: [{}];", self.name.0, self.ty.0)
        } else {
            write!(f, "$arg: {ref_s}[{}]: [{}];", self.name.0, self.ty.0)
        }
    }
}
