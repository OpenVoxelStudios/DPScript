pub mod array;
pub mod at;
pub mod attr;
pub mod binop;
pub mod block;
pub mod call;
pub mod cond;
pub mod constant;
pub mod err;
pub mod func;
pub mod id;
pub mod import;
pub mod literal;
pub mod loops;
pub mod nbt;
pub mod obj;
pub mod parser;
pub mod ret;
pub mod ty;
pub mod unop;
pub mod util;
pub mod var;

use std::path::PathBuf;

use crate::{
    dpscript::{
        ast::{ast::AST, node::Node},
        lexer::{
            err::{LexerErr, LexerFullErr},
            parser::Lexer,
        },
        tokenizer::Token,
    },
    util::Spanned,
};
use flexstr::SharedStr;
use miette::NamedSource;

pub type Result<T, E = LexerErr> = core::result::Result<T, E>;

pub struct FullLexer {
    pub module: SharedStr,
    pub file_name: SharedStr,
    pub source: Vec<char>,
    pub source_str: SharedStr,
    pub named_src: NamedSource<SharedStr>,
    pub namespace: SharedStr,
    pub inner: Lexer,
}

impl FullLexer {
    pub fn new(
        module: SharedStr,
        namespace: SharedStr,
        file_name: SharedStr,
        source: SharedStr,
        keep: bool,
        tokens: Vec<Spanned<Token>>,
    ) -> Self {
        Self {
            module,
            namespace: namespace.clone(),
            named_src: NamedSource::new(&file_name, source.clone()),
            source: source.chars().collect(),
            source_str: source,
            inner: Lexer::new(
                namespace,
                PathBuf::from(&file_name.to_string())
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .trim_end_matches(".dps")
                    .into(),
                keep,
                tokens,
            ),
            file_name,
        }
    }

    pub fn run(self) -> Result<AST, LexerFullErr> {
        let src = self.named_src.clone();
        let module = self.module.clone();

        self.run_inner()
            .map(|it| AST::new(module, src.clone(), it))
            .map_err(|it| LexerFullErr {
                err: vec![it],
                source_code: src,
            })
    }

    fn run_inner(self) -> Result<Vec<Node>> {
        let nodes = self.inner.parse_top_level()?;

        Ok(nodes)
    }
}
