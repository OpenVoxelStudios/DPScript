use crate::{
    ParseCx,
    inner::Rule,
    parse_err,
    util::{ParserUtil, next_or_die},
};
use ast::{
    binop::{BinaryOpNode, BinaryOperation},
    data::{SourceSpan, SpanUtil},
};
use miette::{Result, Severity};
use pest::iterators::Pairs;

pub fn parse_assign<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<BinaryOpNode<'a>> {
    let lhs = inner.parse_one_next(cx)?;
    let op = next_or_die(cx, inner)?;
    let rhs = inner.parse_one_next(cx)?;
    let op_s = op.as_span();

    let op = match op.as_rule() {
        Rule::_assign_eq => BinaryOperation::Assign,
        Rule::_assign_sub_eq => BinaryOperation::SubAssign,
        Rule::_assign_add_eq => BinaryOperation::AddAssign,
        Rule::_assign_div_eq => BinaryOperation::DivAssign,
        Rule::_assign_mul_eq => BinaryOperation::MulAssign,
        Rule::_assign_mod_eq => BinaryOperation::ModAssign,

        _ => {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "invalid::assign_op",
                labels = vec![op_s.label()],
                "Invalid assignment operation: {}",
                op.as_str()
            );
        }
    };

    Ok(BinaryOpNode {
        span,
        op: (op, op_s.into()),
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    })
}

pub fn parse_binop<'a>(
    cx: &mut ParseCx<'a>,
    span: SourceSpan,
    inner: &mut Pairs<'a, Rule>,
) -> Result<BinaryOpNode<'a>> {
    let lhs = inner.parse_one_next(cx)?;
    let op = next_or_die(cx, inner)?;
    let rhs = inner.parse_one_next(cx)?;
    let op_s = op.as_span();

    let op = match op.as_rule() {
        Rule::_bl_op_and => BinaryOperation::CondAnd,
        Rule::_bl_op_or => BinaryOperation::CondOr,
        Rule::_bl_op_eq => BinaryOperation::CondEq,
        Rule::_bl_op_ne => BinaryOperation::CondNeq,
        Rule::_bl_op_lt => BinaryOperation::CondLt,
        Rule::_bl_op_gt => BinaryOperation::CondGt,
        Rule::_bl_op_le => BinaryOperation::CondLe,
        Rule::_bl_op_ge => BinaryOperation::CondGe,

        Rule::_bn_op_mul => BinaryOperation::Mul,
        Rule::_bn_op_div => BinaryOperation::Div,
        Rule::_bn_op_add => BinaryOperation::Add,
        Rule::_bn_op_sub => BinaryOperation::Sub,
        Rule::_bn_op_mod => BinaryOperation::Mod,

        _ => {
            parse_err!(
                cx,
                severity = Severity::Error,
                code = "invalid::binary_op",
                labels = vec![op_s.label()],
                "Invalid binary operation: {}",
                op.as_str()
            );
        }
    };

    Ok(BinaryOpNode {
        span,
        op: (op, op_s.into()),
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    })
}
