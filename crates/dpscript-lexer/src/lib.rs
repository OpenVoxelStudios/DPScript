use crate::{cx::ParseCx, err::WrappedError, parsers::defs::parse_def, util::TokenCursor};
use dpscript_ast::prelude::{Spanned, def::Def};
use dpscript_core::bt::get_backtrace;
use dpscript_parser::Token;

pub mod cx;
pub mod err;
pub mod parsers;
pub mod util;

pub type Result<T, E = err::Error> = core::result::Result<T, E>;

fn parse_inner<'a>(tokens: Vec<Spanned<Token<'a>>>) -> Result<Vec<Def<'a>>> {
    let mut cursor = TokenCursor::new(tokens);
    let mut defs = Vec::new();
    let mut cx = ParseCx::default();

    while cursor.has_next() {
        defs.push(parse_def(&mut cursor, &mut cx)?);
    }

    cursor.assert_empty()?;

    Ok(defs)
}

pub fn parse<'a>(tokens: Vec<Spanned<Token<'a>>>) -> Result<Vec<Def<'a>>, WrappedError> {
    match parse_inner(tokens) {
        Ok(it) => Ok(it),

        Err(err) => Err(WrappedError {
            inner: err,
            backtrace: get_backtrace(),
        }),
    }
}
