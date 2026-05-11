use crate::{cx::ParseCx, parsers::defs::parse_def, util::TokenCursor};
use dpscript_ast::prelude::{Spanned, def::Def};
use dpscript_parser::Token;

pub mod cx;
pub mod err;
pub mod parsers;
pub mod util;

pub type Result<T, E = err::Error> = core::result::Result<T, E>;

pub fn parse<'a>(tokens: Vec<Spanned<Token<'a>>>) -> Result<Vec<Def<'a>>> {
    let mut cursor = TokenCursor::new(tokens);
    let mut defs = Vec::new();
    let mut cx = ParseCx::default();

    while cursor.has_next() {
        defs.push(parse_def(&mut cursor, &mut cx)?);
    }

    cursor.assert_empty()?;

    Ok(defs)
}
