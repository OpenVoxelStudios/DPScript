pub mod array;
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

use crate::{
    dpscript::{
        ast::node::Node,
        lexer::{
            err::{LexerErr, LexerFullErr},
            parser::Lexer,
        },
        tokenizer::Token,
    },
    util::Spanned,
};
use miette::NamedSource;

pub type Result<T, E = LexerErr> = core::result::Result<T, E>;

pub struct FullLexer {
    pub file_name: String,
    pub source: Vec<char>,
    pub source_str: String,
    pub named_src: NamedSource<String>,
    pub namespace: String,
    pub inner: Lexer,
}

impl FullLexer {
    pub fn new(
        namespace: String,
        file_name: String,
        source: String,
        tokens: Vec<Spanned<Token>>,
    ) -> Self {
        Self {
            namespace: namespace.clone(),
            named_src: NamedSource::new(&file_name, source.clone()),
            file_name,
            source: source.chars().collect(),
            source_str: source,
            inner: Lexer::new(namespace, tokens),
        }
    }

    pub fn run(self) -> Result<Vec<Node>, LexerFullErr> {
        let src = self.named_src.clone();

        self.run_inner().map_err(|it| LexerFullErr {
            err: vec![it],
            source_code: src,
        })
    }

    fn run_inner(self) -> Result<Vec<Node>> {
        let nodes = self.inner.parse_top_level()?;

        Ok(nodes)
    }
}
