use crate::{
    dpscript::{
        ast::{ast::Scope, node::Node},
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
        const Inline = 1;
        const Facade = 2;
        const Compiler = 3;
        const Public = 4;
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct FunctionNode {
    pub span: SourceSpan,
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub return_type: Option<TypeRef>,
    pub ident: Identifier,
    pub body: Vec<Node>,
    pub flags: FuncFlags,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, HasSpan)]
pub struct FunctionArg {
    pub span: SourceSpan,
    pub name: String,
    pub ty: TypeRef,
    pub location: DataLocation,
    pub is_this: bool,
}

impl NodeInfo for FunctionNode {
    // It's a function declaration and therefore has no value!
    fn is_const(&self, _scope: &Scope) -> bool {
        false
    }
}
