use crate::{
    Result,
    cx::ParseCx,
    err::Error,
    parsers::{
        blocks::parse_block, constant::parse_constant, enums::parse_enum, func::parse_func,
        import::parse_import, meta::parse_def_meta, objective::parse_objective,
        structs::parse_struct, types::parse_typedef,
    },
    util::TokenCursor,
};
use dpscript_ast::prelude::def::{Def, DefTrait};

#[tracing::instrument(level = tracing::Level::DEBUG)]
fn parse_def_inner<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Def<'a>> {
    c.save();

    match parse_block(c, cx) {
        Ok(it) => return Ok(Def::Block(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_constant(c, cx) {
        Ok(it) => return Ok(Def::Constant(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_enum(c, cx) {
        Ok(it) => return Ok(Def::Enum(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_func(c, cx) {
        Ok(it) => return Ok(Def::Function(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_objective(c, cx) {
        Ok(it) => return Ok(Def::Objective(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_struct(c, cx) {
        Ok(it) => return Ok(Def::Struct(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_typedef(c, cx) {
        Ok(it) => return Ok(Def::Typedef(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    c.save();

    match parse_import(c, cx) {
        Ok(it) => return Ok(Def::Import(it)),
        Err(Error::Skip) => c.restore(),
        Err(other) => return Err(other),
    }

    Err(cx.unexpected(c.take_next()?))
}

#[tracing::instrument(level = tracing::Level::DEBUG)]
pub fn parse_def<'a>(c: &mut TokenCursor<'a>, cx: &mut ParseCx<'a>) -> Result<Def<'a>> {
    let meta = parse_def_meta(c, cx)?;
    let def = parse_def_inner(c, cx)?;

    Ok(def.with_meta(meta))
}
