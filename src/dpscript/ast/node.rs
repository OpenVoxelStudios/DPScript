use crate::dpscript::{
    ast::{binop::BinaryOpNode, constant::ConstantNode, func::FunctionNode, unop::UnaryOpNode},
    check::CheckConst,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Node {
    Constant(ConstantNode),
    Function(FunctionNode),
    UnaryOp(UnaryOpNode),
    BinaryOp(BinaryOpNode),
}

impl CheckConst for Node {
    fn is_const(&self) -> bool {
        match self {
            Self::Constant(me) => me.is_const(),
            Self::Function(me) => me.is_const(),
            Self::UnaryOp(me) => me.is_const(),
            Self::BinaryOp(me) => me.is_const(),
        }
    }
}
