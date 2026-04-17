#![feature(trim_prefix_suffix)]

mod easy;
mod func;
mod import;
mod inner;
mod macros;
mod nbt;
mod ops;
mod parser;
mod util;

use crate::{
    inner::{PestParser, Rule},
    parser::parse_next,
};
use ast::{
    ast::AST,
    data::{NamedSource, SourceSpan},
    loc::{DataLocation, Identifier},
};
use miette::{IntoDiagnostic, Result};
use pest::{Parser, iterators::Pairs};

pub struct FileParser<'a> {
    file: &'a str,
    code: &'a str,
    module: &'a str,
    namespace: &'a str,
    pairs: Pairs<'a, Rule>,
}

struct ParseCx<'a> {
    file: &'a str,
    code: &'a str,
    module: &'a str,
    namespace: &'a str,
    block_id: usize,
    locals: Vec<usize>,
    block_stack: Vec<usize>,

    pub pos: SourceSpan,
}

impl<'a> ParseCx<'a> {
    pub(crate) fn local_var(&mut self) -> DataLocation<'a> {
        let block_ident = format!(
            "{}/blocks/{}/locals",
            self.module.replace("::", "/"),
            self.cur_block().expect("No block context was given!")
        );

        let ident = self.ident(block_ident);

        let locals = self
            .locals
            .last_mut()
            .expect("No block context was set up for local variable counter!");

        let local_id = *locals;

        *locals += 1;

        let local_name = format!("local_{local_id}");

        DataLocation {
            storage: ident,

            // FIXME: This is probably VERY bad. But, this data should live until codegen is done, so...
            path: Box::leak(local_name.into_boxed_str()),
        }
    }

    pub(crate) fn start_block(&mut self) -> Identifier<'a> {
        self.block_id += 1;

        let b = self.block_id - 1;

        self.block_stack.push(b);
        self.locals.push(0);

        let id = format!("{}/blocks/{}", self.module.replace("::", "/"), b);

        self.ident(id)
    }

    pub(crate) fn end_block(&mut self) -> usize {
        self.locals
            .pop()
            .expect("Local variable counter not set up!");
        self.block_stack.pop().expect("Block wasn't started!")
    }

    pub(crate) fn cur_block(&self) -> Option<usize> {
        self.block_stack.last().copied()
    }

    pub(crate) fn ident(&self, name: impl AsRef<str>) -> Identifier<'a> {
        Identifier {
            namespace: self.namespace,

            // FIXME: This is probably VERY bad. But, this data should live until codegen is done, so...
            path: Box::leak(name.as_ref().to_string().into_boxed_str()),
        }
    }
}

impl<'a> FileParser<'a> {
    fn new(file: &'a str, module: &'a str, namespace: &'a str, code: &'a str) -> Result<Self> {
        Ok(Self {
            file,
            module,
            namespace,
            code,
            pairs: PestParser::parse(Rule::body, code).into_diagnostic()?,
        })
    }

    fn run(&mut self) -> Result<AST<'a>> {
        let mut ast = Vec::new();

        let mut cx = ParseCx {
            file: self.file,
            module: self.module,
            code: self.code,
            namespace: self.namespace,
            pos: SourceSpan::new(0, 0),
            block_id: 0,
            block_stack: Vec::new(),
            locals: Vec::new(),
        };

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

        Ok(full)
    }

    pub fn parse(
        file: &'a str,
        module: &'a str,
        namespace: &'a str,
        code: &'a str,
    ) -> Result<AST<'a>> {
        Ok(Self::new(file, module, namespace, code)?.run()?)
    }
}
