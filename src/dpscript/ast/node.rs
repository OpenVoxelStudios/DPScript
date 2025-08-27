use crate::dpscript::ast::{constant::ConstantNode, func::FunctionNode};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Node {
    Constant(ConstantNode),
    Function(FunctionNode),
}
