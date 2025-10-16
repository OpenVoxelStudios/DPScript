use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::binop::{BinaryOpNode, BinaryOperation},
        data::NodeInfo,
        ty::{BuiltInType, TypeRef},
        validator::{Result, Validator, err::Err},
    },
};

impl Validator {
    pub fn validate_binop(&mut self, node: &BinaryOpNode) -> Result<()> {
        if node.op == BinaryOperation::Field {
            if node.returns(self.scope()?).is_none() {
                self.errors
                    .push(Err::CannotComputeType { span: node.span() });
            }

            return Ok(());
        }

        if node.op == BinaryOperation::ArrayIndex {
            let Some(lhs_ty) = node.lhs.returns(self.scope()?) else {
                self.errors.push(Err::CannotComputeType {
                    span: node.lhs.span(),
                });

                return Ok(());
            };

            let Some(rhs_ty) = node.rhs.returns(self.scope()?) else {
                self.errors.push(Err::CannotComputeType {
                    span: node.rhs.span(),
                });

                return Ok(());
            };

            if !lhs_ty.is_array() {
                self.errors.push(Err::NotAnArray { span: node.span() });
            }

            match rhs_ty {
                TypeRef::BuiltIn(BuiltInType::Int) => (),
                _ => {
                    self.errors.push(Err::NonIntIndex {
                        span: node.rhs.span(),
                    });
                }
            }

            return Ok(());
        }

        let Some(lhs_ty) = node.lhs.returns(self.scope()?) else {
            self.errors.push(Err::CannotComputeType {
                span: node.lhs.span(),
            });

            return Ok(());
        };

        let Some(rhs_ty) = node.rhs.returns(self.scope()?) else {
            self.errors.push(Err::CannotComputeType {
                span: node.rhs.span(),
            });

            return Ok(());
        };

        let compatible = match node.op {
            BinaryOperation::Add => {
                (lhs_ty.is_numeric() && rhs_ty.is_numeric()) || (lhs_ty.is_nbt() && rhs_ty.is_nbt())
            }

            BinaryOperation::AddAssign => {
                (lhs_ty.is_numeric() && rhs_ty.is_numeric()) || (lhs_ty.is_nbt() && rhs_ty.is_nbt())
            }

            BinaryOperation::Sub => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::Mul => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::Div => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::Mod => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::BitAnd => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::BitOr => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::BitXor => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::CondGt => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::CondGe => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::CondLt => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::CondLe => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::SubAssign => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::MulAssign => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::DivAssign => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::ModAssign => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::BitAndAssign => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::BitOrAssign => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::BitXorAssign => lhs_ty.is_numeric() && rhs_ty.is_numeric(),
            BinaryOperation::CondAnd => lhs_ty == rhs_ty || lhs_ty.is_any() || rhs_ty.is_any(),
            BinaryOperation::CondOr => lhs_ty == rhs_ty || lhs_ty.is_any() || rhs_ty.is_any(),
            BinaryOperation::CondEq => lhs_ty == rhs_ty || lhs_ty.is_any() || rhs_ty.is_any(),
            BinaryOperation::CondNeq => lhs_ty == rhs_ty || lhs_ty.is_any() || rhs_ty.is_any(),

            BinaryOperation::Assign => {
                lhs_ty == rhs_ty
                    || lhs_ty.is_any()
                    || rhs_ty.is_any()
                    || (lhs_ty.is_numeric() && rhs_ty.is_numeric())
                    || (lhs_ty.is_nbt() && rhs_ty.is_nbt())
                    || (lhs_ty == TypeRef::BuiltIn(BuiltInType::Transform)
                        && match &rhs_ty {
                            TypeRef::Array(it) => it.is_numeric(),
                            TypeRef::SizedArray(it, _) => it.is_numeric(),
                            _ => false,
                        })
                    || (rhs_ty == TypeRef::BuiltIn(BuiltInType::Transform)
                        && match &lhs_ty {
                            TypeRef::Array(it) => it.is_numeric(),
                            TypeRef::SizedArray(it, _) => it.is_numeric(),
                            _ => false,
                        })
            }

            BinaryOperation::Range => lhs_ty == rhs_ty && !lhs_ty.is_any() && !rhs_ty.is_any(),
            BinaryOperation::Field => node.lhs.is_ident() || node.lhs.is_binary_op(),

            BinaryOperation::ArrayIndex => {
                (node.lhs.is_ident() || node.lhs.is_binary_op()) && rhs_ty.is_numeric()
            }
        };

        if !compatible {
            self.errors.push(Err::IncompatibleTypes {
                span: node.span(),
                lhs: lhs_ty,
                rhs: rhs_ty,
            });
        }

        Ok(())
    }
}
