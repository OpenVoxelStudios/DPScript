use crate::{Result, cx::ParseCx, err::Error, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::value::binop::{BinOp, BoolOp, MathOp, Operation};
use dpscript_parser::{Comparison, Operator as TOperator, Token};

pub fn parse_binop<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<'a, BinOp<'a>> {
    c.begin_span();

    let mut lhs_buf = Vec::new();
    let mut op = None;

    while let Some(tkn) = c.next() {
        match tkn.0 {
            Token::Operator(TOperator::Plus) => {
                op = Some(Operation::Math(MathOp::Add));
                break;
            }

            Token::Operator(TOperator::Minus) => {
                op = Some(Operation::Math(MathOp::Sub));
                break;
            }

            Token::Operator(TOperator::Star) => {
                op = Some(Operation::Math(MathOp::Mul));
                break;
            }

            Token::Operator(TOperator::Slash) => {
                op = Some(Operation::Math(MathOp::Div));
                break;
            }

            Token::Operator(TOperator::Percent) => {
                op = Some(Operation::Math(MathOp::Mod));
                break;
            }

            Token::Comparison(Comparison::Equal) => {
                op = Some(Operation::Bool(BoolOp::Eq));
                break;
            }

            Token::Comparison(Comparison::NotEqual) => {
                op = Some(Operation::Bool(BoolOp::NotEq));
                break;
            }

            Token::Comparison(Comparison::Greater) => {
                op = Some(Operation::Bool(BoolOp::Greater));
                break;
            }

            Token::Comparison(Comparison::GreaterEqual) => {
                op = Some(Operation::Bool(BoolOp::GreaterEq));
                break;
            }

            Token::Comparison(Comparison::Less) => {
                op = Some(Operation::Bool(BoolOp::Less));
                break;
            }

            Token::Comparison(Comparison::LessEqual) => {
                op = Some(Operation::Bool(BoolOp::LessEq));
                break;
            }

            _ => lhs_buf.push(tkn),
        }
    }

    let mut lhs_buf = TokenCursor::new(lhs_buf);
    let lhs = parse_value(&mut lhs_buf, cx)?;

    lhs_buf.assert_empty()?;

    let rhs = parse_value(c, cx)?;
    let span = c.end_span();

    Ok(BinOp {
        lhs: Box::new(lhs),
        op: op.ok_or(Error::MissingOp { span: span.into() })?,
        rhs: Box::new(rhs),
        span,
    })
}
