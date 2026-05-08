use crate::{Result, cx::ParseCx, util::TokenCursor};
use dpscript_ast::prelude::value::Value;

pub fn parse_value<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<'a, Value<'a>> {
    todo!()
}
