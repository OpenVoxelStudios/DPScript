use crate::{
    common::traits::Validated,
    dpscript::{
        ast::{ast::Module, node::Node},
        check::CheckConst,
        ty::TypeRef,
    },
    util::Spanned,
};
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ConstantNode {
    pub span: SourceSpan,
    pub name: String,
    pub ty: Option<TypeRef>,
    pub value: Vec<Node>,
}

impl ConstantNode {}

impl Validated for ConstantNode {
    fn validate(
        &self,
        module: &Module,
        warnings: &mut Vec<Spanned<String>>,
        errors: &mut Vec<Spanned<String>>,
    ) -> Result<(), ()> {
        if self.ty.is_none() {
            errors.push(("Constant must have a declared type!".into(), self.span));

            Err(())
        } else {
            Ok(())
        }
    }
}

impl CheckConst for ConstantNode {
    // It's a variable declaration and therefore has no value!
    fn is_const(&self) -> bool {
        false
    }
}
