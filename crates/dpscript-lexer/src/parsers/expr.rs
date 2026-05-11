use crate::{
    Result, cx::ParseCx, err::Error, parsers::{
        assign::parse_assign, at::parse_at_block, call::parse_call, cond::parse_cond, constant::parse_constant, loops::parse_for_loop, ret::parse_ret, var::parse_var
    }, util::TokenCursor
};
use dpscript_ast::prelude::expr::Expr;
use dpscript_parser::{Punct, Token};

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_expr<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Expr<'a>> {
    c.save();

    match parse_at_block(c, cx) {
        Ok(it) => return Ok(Expr::At(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_cond(c, cx) {
        Ok(it) => return Ok(Expr::Cond(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_for_loop(c, cx) {
        Ok(it) => return Ok(Expr::ForLoop(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    // TODO: Infinite loop, while loop

    c.save();

    match parse_call(c, cx) {
        Ok(it) => {
            c.expect(Token::Punct(Punct::Semi))?;
            return Ok(Expr::Call(it));
        }

        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_var(c, cx) {
        Ok(it) => return Ok(Expr::Variable(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_ret(c, cx) {
        Ok(it) => return Ok(Expr::Return(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_constant(c, cx) {
        Ok(it) => return Ok(Expr::Constant(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_assign(c, cx) {
        Ok(it) => return Ok(Expr::Assign(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    Err(cx.unexpected(c.take_next()?))
}
