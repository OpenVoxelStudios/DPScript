use crate::dpscript::{
    ast::{
        binop::{BinaryOpNode, BinaryOperation},
        node::Node,
    },
    data::NodeInfo,
    validator::{Result, Validator},
};

impl Validator {
    pub fn validate_binop(&mut self, node: &BinaryOpNode) -> Result<()> {
        // if node.op.is_assign() {

        // }

        // let lhs = node.lhs.returns(self.scope()?);
        // let rhs = node.rhs.returns(self.scope()?);

        // TODO

        Ok(())
    }
}
