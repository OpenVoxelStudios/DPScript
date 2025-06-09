mod assign;
mod attr;
mod block;
mod call;
mod cond;
mod enums;
mod export;
mod func;
mod import;
mod literal;
mod loops;
mod module;
mod objective;
mod ops;
mod ret;
mod sub;
mod ty;
mod var;

pub use assign::*;
pub use attr::*;
pub use block::*;
pub use call::*;
pub use cond::*;
pub use enums::*;
pub use export::*;
pub use func::*;
pub use import::*;
pub use literal::*;
pub use loops::*;
pub use module::*;
pub use objective::*;
pub use ops::*;
pub use ret::*;
pub use sub::*;
pub use ty::*;
pub use var::*;

use crate::Spanned;
use miette::SourceSpan;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub enum Node {
    Assign(Assign),
    Module(Module),
    Literal(Literal),
    Ident(Spanned<String>),
    Import(Import),
    Function(Function),
    Variable(Variable),
    Call(Call),
    Operation(Operation),
    Block(Block),
    Loop(Loop),
    Enum(Enum),
    Return(Return),
    Objective(Objective),
    Conditional(Conditional),
    Export(Export),
    Subroutine(Subroutine),
    Goto(Spanned<String>),
}

#[derive(Debug, Clone, Serialize)]
pub enum TopLevelNode {
    Module(Module),
    Import(Import),
    Function(Function),
    Variable(Variable),
    Block(Block),
    Enum(Enum),
    Objective(Objective),
    Export(Export),
}

impl Node {
    pub fn span(&self) -> SourceSpan {
        match self {
            Self::Assign(a) => a.span,
            Self::Module(m) => m.span,
            Self::Literal(l) => l.span(),
            Self::Ident(i) => i.1,
            Self::Import(i) => i.span,
            Self::Function(f) => f.span,
            Self::Variable(v) => v.span,
            Self::Call(c) => c.span,
            Self::Operation(o) => o.span,
            Self::Block(b) => b.span,
            Self::Loop(l) => l.span,
            Self::Enum(e) => e.span,
            Self::Return(r) => r.span,
            Self::Objective(o) => o.span,
            Self::Conditional(c) => c.span,
            Self::Export(e) => e.span,
            Self::Subroutine(s) => s.span,
            Self::Goto(g) => g.1,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Assign(_) => write!(f, "Assign"),
            Self::Module(_) => write!(f, "Module"),
            Self::Literal(_) => write!(f, "Literal"),
            Self::Ident(_) => write!(f, "Import"),
            Self::Function(_) => write!(f, "Function"),
            Self::Import(_) => write!(f, "Import"),
            Self::Variable(_) => write!(f, "Variable"),
            Self::Call(_) => write!(f, "Call"),
            Self::Operation(_) => write!(f, "Operation"),
            Self::Block(_) => write!(f, "Block"),
            Self::Loop(_) => write!(f, "Loop"),
            Self::Enum(_) => write!(f, "Enum"),
            Self::Return(_) => write!(f, "Return"),
            Self::Objective(_) => write!(f, "Objective"),
            Self::Conditional(_) => write!(f, "Conditional"),
            Self::Export(_) => write!(f, "Export"),
            Self::Subroutine(_) => write!(f, "Subroutine"),
            Self::Goto(_) => write!(f, "Goto"),
        }
    }
}

impl TopLevelNode {
    pub fn span(&self) -> SourceSpan {
        match self {
            Self::Module(m) => m.span,
            Self::Import(i) => i.span,
            Self::Function(f) => f.span,
            Self::Variable(v) => v.span,
            Self::Block(b) => b.span,
            Self::Enum(e) => e.span,
            Self::Objective(o) => o.span,
            Self::Export(e) => e.span,
        }
    }
}

impl fmt::Display for TopLevelNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Module(_) => write!(f, "Module"),
            Self::Function(_) => write!(f, "Function"),
            Self::Import(_) => write!(f, "Import"),
            Self::Variable(_) => write!(f, "Variable"),
            Self::Block(_) => write!(f, "Block"),
            Self::Enum(_) => write!(f, "Enum"),
            Self::Objective(_) => write!(f, "Objective"),
            Self::Export(_) => write!(f, "Export"),
        }
    }
}
