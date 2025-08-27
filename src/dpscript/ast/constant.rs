use miette::SourceSpan;

use crate::{
    common::traits::Validated,
    dpscript::{ast::node::Node, ty::TypeRef},
    util::{DataLocation, Spanned},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ConstantNode {
    pub span: SourceSpan,
    pub name: String,
    pub ty: Option<TypeRef>,
    pub location: DataLocation,
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
