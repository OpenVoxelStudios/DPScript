use crate::{
    chain_parsers,
    dpscript::{
        ast::node::Node,
        lexer::{call::CallLexer, constant::ConstLexer, err::LexerErr, id::IdentLexer, util::LexerMethods, Result},
        tokenizer::Token,
    },
    impl_lexer,
    util::Spanned,
};
use miette::{SourceOffset, SourceSpan};

macro_rules! lexer {
    (($value: expr): $name: ident => [$($func: ident),*]) => {
        pub struct $name {
            pub tokens: Vec<Spanned<Token>>,
            pub pos: usize,
            pub namespace: String,
            pub last_pos: SourceSpan,
            pub stack: Vec<usize>,
        }

        impl_lexer!($name);

        impl $name {
            pub fn new(namespace: String, tokens: Vec<Spanned<Token>>) -> Self {
                let last = tokens
                    .first()
                    .map(|it| it.1.clone())
                    .unwrap_or(SourceSpan::new(SourceOffset::from_location("", 0, 0), 0));

                Self {
                    namespace,
                    tokens,
                    pos: 0,
                    last_pos: last,
                    stack: Vec::new(),
                }
            }

            pub fn read_node(&mut self) -> Result<Node> {
                chain_parsers!(($value): self; [
                    $($func),*
                ]);
            }

            pub fn read_node_sep(&mut self, sep: &Token) -> Result<Option<Node>> {
                chain_parsers!(sep, ($value): self; [
                    $($func),*
                ]);
            }

            pub fn parse(mut self) -> Result<Vec<Node>> {
                let mut nodes = Vec::new();

                while self.has_next() {
                    nodes.push(self.read_node()?);
                }

                Ok(nodes)
            }

            pub fn parse_sep(mut self, sep: Token) -> Result<Vec<Node>> {
                let mut nodes = Vec::new();

                while self.has_next() {
                    if let Some(node) = self.read_node_sep(&sep)? {
                        nodes.push(node);
                    }
                }

                Ok(nodes)
            }
        }
    };
}

lexer!((false): TopLevelLexer => [
    read_import,
    read_func,
    read_init_block,
    read_tick_block,
    read_objective,
    read_const
]);

lexer!((false): BodyLexer => [
    read_const,
    read_var,
    read_call,
    read_for_loop,
    read_cond,
    read_return
]);

lexer!((true): ValueLexer => [
    read_literal,
    read_array,
    read_call,
    read_unop,
    read_ident_full
]);
