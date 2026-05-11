use crate::{Result, cx::ParseCx, err::Error, parsers::value::parse_value, util::TokenCursor};
use dpscript_ast::prelude::expr::assign::{Assign, AssignOp};
use dpscript_parser::{Assignment, Punct, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_assign<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Assign<'a>> {
    c.begin_span();

    let mut lhs_buf = Vec::new();
    let mut op = None;

    while let Some(tkn) = c.next() {
        match tkn.0 {
            Token::Assignment(Assignment::Equal) => {
                op = Some(AssignOp::Eq);
                break;
            }

            Token::Assignment(Assignment::AddEqual) => {
                op = Some(AssignOp::AddEq);
                break;
            }

            Token::Assignment(Assignment::SubEqual) => {
                op = Some(AssignOp::SubEq);
                break;
            }

            Token::Assignment(Assignment::MulEqual) => {
                op = Some(AssignOp::MulEq);
                break;
            }

            Token::Assignment(Assignment::DivEqual) => {
                op = Some(AssignOp::DivEq);
                break;
            }

            Token::Assignment(Assignment::ModEqual) => {
                op = Some(AssignOp::ModEq);
                break;
            }

            Token::Punct(Punct::Semi) => {
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

    // c.expect(Token::Punct(Punct::Semi))?;

    let span = c.end_span();

    Ok(Assign {
        lhs: Box::new(lhs),
        op: op.ok_or(Error::MissingOp { span: span.into() })?,
        rhs: Box::new(rhs),
        span,
    })
}
