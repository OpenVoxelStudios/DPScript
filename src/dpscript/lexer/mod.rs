mod analysis;

use crate::{AST, Cursor, Node, Result, Spanned, Token};
use analysis::Analyzer;
use miette::NamedSource;

pub type TokenCursor = Cursor<Vec<Spanned<Token>>, NamedSource<String>>;

#[derive(Debug, Clone)]
pub struct Lexer {
    pub file: String,
    pub source: NamedSource<String>,
    pub tokens: Vec<Spanned<Token>>,

    nodes: Vec<Node>,
}

impl Lexer {
    pub fn new(
        file: impl AsRef<str>,
        source: impl AsRef<str>,
        tokens: Vec<Spanned<Token>>,
    ) -> Self {
        Lexer {
            file: file.as_ref().into(),
            source: NamedSource::new(file, source.as_ref().into()),
            tokens,
            nodes: Vec::new(),
        }
    }

    pub fn run(mut self) -> Result<Self> {
        let mut cursor =
            TokenCursor::new_from_src(&self.file, self.source.inner().clone(), self.tokens.clone());

        while let Some(item) = cursor.next() {
            Node::analyze(item, &mut cursor, &mut self.nodes)?;
        }

        Ok(self)
    }

    pub fn ast(self) -> AST {
        AST {
            nodes: self.nodes,
            indexed: false,
            cached: false,
            modules: None,
            top_level: None,
            blocks: None,
            enums: None,
            exports: None,
            funcs: None,
            imports: None,
            objectives: None,
            vars: None,
            export_nodes: None,
        }
    }
}
