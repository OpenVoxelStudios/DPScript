use crate::{Result, cx::ParseCx, err::Error, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::value::binop::{BinOp, BoolOp, MathOp, Operation};
use dpscript_parser::{BraceType, Comparison, Operator as TOperator, Punct, Token};

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_binop<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<BinOp<'a>> {
    // TODO: Some sort of PEMDAS here with chains of binary operations

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

            Token::Comparison(Comparison::And) => {
                op = Some(Operation::Bool(BoolOp::And));
                break;
            }

            Token::Comparison(Comparison::Or) => {
                op = Some(Operation::Bool(BoolOp::Or));
                break;
            }

            Token::BraceGroup(BraceType::Braces, _) | Token::Punct(Punct::Semi | Punct::Comma) => {
                return Err(cx.skip());
            }

            _ => lhs_buf.push(tkn),
        }
    }

    if lhs_buf.is_empty() || op.is_none() {
        return Err(cx.skip());
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
        resolved: None,
    })
}

// #[tracing::instrument(level = tracing::Level::DEBUG)]
// pub fn parse_binop<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<BinOp<'a>> {
//     // TODO: Some sort of PEMDAS here with chains of binary operations

//     c.begin_span();

//     let mut lhs_buf = Vec::new();
//     let mut rhs_buf = Vec::new();
//     let mut is_rhs = false;
//     let mut op = None;
//     let mut last_op = None;

//     while let Some(tkn) = c.next() {
//         match tkn.0 {
//             Token::Operator(TOperator::Plus) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Math(MathOp::Add));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Operator(TOperator::Minus) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Math(MathOp::Sub));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Operator(TOperator::Star) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Math(MathOp::Mul));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Operator(TOperator::Slash) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Math(MathOp::Div));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Operator(TOperator::Percent) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Math(MathOp::Mod));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Comparison(Comparison::Equal) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Bool(BoolOp::Eq));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Comparison(Comparison::NotEqual) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Bool(BoolOp::NotEq));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Comparison(Comparison::Greater) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Bool(BoolOp::Greater));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Comparison(Comparison::GreaterEqual) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Bool(BoolOp::GreaterEq));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Comparison(Comparison::Less) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Bool(BoolOp::Less));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::Comparison(Comparison::LessEqual) => {
//                 if let Some(last_op) = last_op {
//                     lhs_buf.push(last_op);
//                 }

//                 last_op = Some(tkn);
//                 op = Some(Operation::Bool(BoolOp::LessEq));
//                 lhs_buf.append(&mut rhs_buf);
//                 is_rhs = true;
//             }

//             Token::BraceGroup(BraceType::Braces, _) | Token::Punct(Punct::Semi | Punct::Comma) => {
//                 c.back();
//                 break;
//             }

//             _ => {
//                 if is_rhs {
//                     rhs_buf.push(tkn)
//                 } else {
//                     lhs_buf.push(tkn)
//                 }
//             }
//         }
//     }

//     if lhs_buf.is_empty() || rhs_buf.is_empty() || op.is_none() {
//         return Err(cx.skip());
//     }

//     let mut lhs_buf = TokenCursor::new(lhs_buf);
//     let lhs = parse_value(&mut lhs_buf, cx)?;

//     lhs_buf.assert_empty()?;

//     let mut rhs_buf = TokenCursor::new(rhs_buf.to_vec());
//     let rhs = parse_value(&mut rhs_buf, cx)?;

//     rhs_buf.assert_empty()?;

//     let span = c.end_span();

//     Ok(BinOp {
//         lhs: Box::new(lhs),
//         op: op.ok_or(Error::MissingOp { span: span.into() })?,
//         rhs: Box::new(rhs),
//         span,
//     })
// }

#[dpscript_core::trace_fn_lexer]
#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_array_index<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<BinOp<'a>> {
    c.begin_span();

    if c.check(&Token::Operator(TOperator::At)) {
        return Err(cx.skip());
    }

    let mut lhs_buf = Vec::new();
    let mut rhs_buf = None;

    while let Some(tkn) = c.next() {
        match tkn.0 {
            Token::BraceGroup(BraceType::Brackets, group) => {
                rhs_buf = Some(group);
                break;
            }

            Token::BraceGroup(BraceType::Braces, _) | Token::Punct(Punct::Semi | Punct::Comma) => {
                return Err(cx.skip());
            }

            _ => lhs_buf.push(tkn),
        }
    }

    if lhs_buf.is_empty() {
        return Err(cx.skip());
    }

    let Some(rhs_buf) = rhs_buf else {
        return Err(cx.skip());
    };

    let mut lhs_buf = TokenCursor::new(lhs_buf);
    let lhs = parse_value(&mut lhs_buf, cx)?;

    lhs_buf.assert_empty()?;

    let mut rhs_buf = TokenCursor::new(rhs_buf.to_vec());
    let rhs = parse_value(&mut rhs_buf, cx)?;

    rhs_buf.assert_empty()?;

    let span = c.end_span();

    Ok(BinOp {
        lhs: Box::new(lhs),
        op: Operation::ArrayIndex,
        rhs: Box::new(rhs),
        span,
        resolved: None,
    })
}
