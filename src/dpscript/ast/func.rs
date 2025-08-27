use miette::SourceSpan;
use crate::{dpscript::{ast::node::Node, ty::TypeRef}, util::{DataLocation, Identifier}};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FunctionNode {
    pub span: SourceSpan,
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub return_type: Option<TypeRef>,
    pub ident: Identifier,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FunctionArg {
    pub span: SourceSpan,
    pub name: String,
    pub ty: TypeRef,
    pub location: DataLocation,
}
