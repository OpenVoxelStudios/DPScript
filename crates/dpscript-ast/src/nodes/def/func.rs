use crate::{
    prelude::{
        SourceSpan,
        expr::Expr,
        meta::{DefFlags, DefMeta},
        types::TypeRef,
    },
    util::Name,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct Function<'a> {
    /// The function's info. This is stored in a seperate struct to make it easy to clone
    /// and move around for type checking and validation (see [crate::nodes::expr::call::ResolvedCall]).
    pub info: FunctionInfo<'a>,
    pub body: Vec<Expr<'a>>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct FunctionArg<'a> {
    pub name: Name<'a>,
    pub ty: TypeRef<'a>,
    pub span: SourceSpan,
    pub meta: DefMeta<'a>,
    pub is_ref: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, HasSpan)]
pub struct FunctionInfo<'a> {
    /// The name of the function.
    pub name: Name<'a>,

    /// The definition's flags.
    pub flags: Vec<DefFlags>,

    /// If the function is an instance function, this is the type it is implemented on.
    pub target: Option<TypeRef<'a>>,

    /// The arguments to the function.
    pub args: Vec<FunctionArg<'a>>,

    /// The optional return type of the function.
    pub ret: Option<TypeRef<'a>>,

    /// The span the function was defined in.
    pub span: SourceSpan,

    /// The definition metadata of the function.
    pub meta: DefMeta<'a>,
}
