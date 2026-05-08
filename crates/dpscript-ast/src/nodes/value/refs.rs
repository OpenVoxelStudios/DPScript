use dpscript_core::SourceSpan;

use crate::{
    prelude::{
        def::{constant::Constant, func::Function},
        expr::var::Variable,
        value::Value,
    },
    util::Name,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct ValueRef<'a> {
    /// The value root.
    /// If this was `(a + b).c.d.e`, this would be `(a + b)`.
    pub root: Box<Value<'a>>,

    /// The field path.
    /// If this was `(a + b).c.d.e`, this would be [c, d, e].
    pub path: Vec<Name<'a>>,

    /// The span of the value ref.
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct VarRef<'a> {
    pub name: Name<'a>,
    pub resolved: Option<VarInfo<'a>>,
    pub span: SourceSpan,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpanGroup)]
pub enum VarInfo<'a> {
    Const(Box<Constant<'a>>),
    Var(Box<Variable<'a>>),

    /// Only used when parsing calls, but this counts as a reference to a "value". Easier to parse things this way.
    Func(Box<Function<'a>>),
}
