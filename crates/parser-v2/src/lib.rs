mod easy;
mod func;
mod import;
mod inner;
mod macros;
mod nbt;
mod ops;
mod parser;
mod util;

use std::fs;

use crate::{
    inner::{PestParser, Rule},
    parser::parse_next,
};
use ast::{
    ast::AST,
    data::{NamedSource, SourceSpan},
};
use miette::{IntoDiagnostic, Result};
use pest::{Parser, iterators::Pairs};
use ron::ser::PrettyConfig;

pub struct FileParser<'a> {
    file: &'a str,
    code: &'a str,
    pairs: Pairs<'a, Rule>,
}

struct ParseCx<'a> {
    file: &'a str,
    code: &'a str,

    pub pos: SourceSpan,
}

impl<'a> FileParser<'a> {
    fn new(file: &'a str, code: &'a str) -> Result<Self> {
        Ok(Self {
            file,
            code,
            pairs: PestParser::parse(Rule::body, code).into_diagnostic()?,
        })
    }

    fn run(&mut self) -> Result<AST<'a>> {
        let mut ast = Vec::new();

        let mut cx = ParseCx {
            file: self.file,
            code: self.code,
            pos: SourceSpan::new(0, 0),
        };

        fs::write("pairs.ron", format!("{:#?}", self.pairs)).unwrap();

        while !self.pairs.is_empty() {
            ast.extend(parse_next(&mut cx, &mut self.pairs)?);
        }

        let full = AST::new(
            &self.file,
            &self.file,
            NamedSource {
                code: &self.code,
                file: &self.file,
            },
            ast,
        );

        fs::write(
            "ast.ron",
            ron::ser::to_string_pretty(
                &full,
                PrettyConfig::new()
                    .compact_arrays(false)
                    .compact_maps(false)
                    .compact_structs(false)
                    .separate_tuple_members(true)
                    .struct_names(true),
            )
            .unwrap(),
        )
        .unwrap();

        Ok(full)
    }

    pub fn parse(file: &'a str, code: &'a str) -> Result<AST<'a>> {
        Ok(Self::new(file, code)?.run()?)
    }
}

pub fn test() -> Result<()> {
    FileParser::parse("sqrt.dps", include_str!("../../../std/src/gm/sqrt.dps"))?;

    Ok(())
}
