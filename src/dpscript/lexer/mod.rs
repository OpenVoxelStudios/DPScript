pub mod attr;
pub mod block;
pub mod err;
pub mod func;
pub mod import;
pub mod util;

use miette::{NamedSource, SourceOffset, SourceSpan};

use crate::{
    dpscript::{ast::ast::AST, lexer::err::LexerErr, tokenizer::Token},
    util::Spanned,
};

pub type Result<T, E = LexerErr> = core::result::Result<T, E>;

pub struct Lexer {
    pub file_name: String,
    pub source: Vec<char>,
    pub source_str: String,
    pub tokens: Vec<Spanned<Token>>,
    pub pos: usize,
    pub named_src: NamedSource<String>,
    last_pos: SourceSpan,
    stack: Vec<usize>,
}

macro_rules! chain_parsers {
    ($self: ident; $nodes: ident = [$($parser: ident),*]) => {
        while $self.has_next() {
            let mut expected = Vec::new();
            let mut span = $self.loc();

            $(
                let res = $self.$parser();

                match res {
                    Ok(node) => {
                        $nodes.push(node);
                        continue;
                    }

                    Err(LexerErr::StartParse { src: _, span: sp, expect, got: _ }) => {
                        expected.push(expect);

                        if sp.len() > span.len() {
                            span = sp;
                        }
                    }

                    Err(other) => return Err(other),
                };
            )*

            return Err(LexerErr::ExpectedAny {
                src: $self.src(),
                span,
                expected: expected
                    .into_iter()
                    .map(|it| format!("'{it}'"))
                    .collect::<Vec<_>>().join(", ")
            });
        }
    };
}

impl Lexer {
    pub fn new(file_name: String, source: String, tokens: Vec<Spanned<Token>>) -> Self {
        let last = tokens
            .first()
            .map(|it| it.1.clone())
            .unwrap_or(SourceSpan::new(
                SourceOffset::from_location(&source, 0, 0),
                0,
            ));

        Self {
            named_src: NamedSource::new(&file_name, source.clone()),
            file_name,
            source: source.chars().collect(),
            source_str: source,
            tokens,
            pos: 0,
            last_pos: last,
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        self.stack.push(self.pos);
    }

    pub fn pop(&mut self) -> Result<()> {
        self.pos = self.stack.pop().ok_or(LexerErr::StackPop {
            src: self.src(),
            span: self.loc(),
        })?;

        Ok(())
    }

    /// The same as [`Self::pop`], but without resetting the position.
    pub fn pop_in_place(&mut self) -> Result<()> {
        self.stack.pop().ok_or(LexerErr::StackPop {
            src: self.src(),
            span: self.loc(),
        })?;

        Ok(())
    }

    pub fn eat(&mut self) -> Option<Spanned<Token>> {
        self.pos += 1;

        if let Some(tkn) = self.tokens.get(self.pos) {
            self.last_pos = tkn.1.clone();
        }

        self.tokens.get(self.pos - 1).cloned()
    }

    pub fn has_next(&self) -> bool {
        self.pos < self.tokens.len()
    }

    pub fn peek(&self, amount: usize) -> Option<&Spanned<Token>> {
        self.tokens.get(self.pos + amount)
    }

    pub fn loc(&self) -> SourceSpan {
        self.last_pos.clone()
    }

    pub fn src(&self) -> NamedSource<String> {
        self.named_src.clone()
    }

    pub fn eof(&self) -> LexerErr {
        LexerErr::EOF {
            span: self.loc(),
            src: self.src(),
        }
    }

    pub fn run(&mut self) -> Result<AST> {
        let ast = AST::new();
        let mut nodes = Vec::new();

        chain_parsers!(self; nodes = [
            read_import,
            read_func,
            read_init_block,
            read_tick_block
        ]);

        dbg!(&nodes);

        Ok(ast)
    }
}
