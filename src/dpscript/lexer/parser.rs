use crate::{
    chain_parsers,
    dpscript::{
        ast::node::Node,
        lexer::{Result, err::LexerErr, id::IdentLexer, util::LexerMethods},
        tokenizer::Token,
    },
    impl_lexer,
    util::Spanned,
};
use miette::{SourceOffset, SourceSpan};

macro_rules! lexer {
    (($value: expr): $name: ident => [$($func: ident),*]) => {
        concat_idents::concat_idents!(name = read_, $name {
            impl Lexer {
                pub fn name(&mut self) -> Result<Node> {
                    chain_parsers!(($value): self; [
                        $($func),*
                    ]);
                }
            }
        });

        concat_idents::concat_idents!(name = read_, $name, _sep {
            impl Lexer {
                pub fn name(&mut self, sep: &Token) -> Result<Option<Node>> {
                    chain_parsers!(sep, ($value): self; [
                        $($func),*
                    ]);
                }
            }
        });

        concat_idents::concat_idents!(name = parse_, $name {
            impl Lexer {
                pub fn name(mut self) -> Result<Vec<Node>> {
                    let mut nodes = Vec::new();

                    while self.has_next() {
                        concat_idents::concat_idents!(func = read_, $name {
                            nodes.push(self.func()?);
                        });
                    }

                    Ok(nodes)
                }
            }
        });

        concat_idents::concat_idents!(name = parse_, $name, _sep {
            impl Lexer {
                pub fn name(mut self, sep: Token) -> Result<Vec<Node>> {
                    let mut nodes = Vec::new();

                    while self.has_next() {
                        concat_idents::concat_idents!(func = read_, $name, _sep {
                            if let Some(node) = self.func(&sep)? {
                                nodes.push(node);
                            }
                        });
                    }

                    Ok(nodes)
                }
            }
        });
    };
}

pub struct Lexer {
    pub tokens: Vec<Spanned<Token>>,
    pub pos: usize,
    pub namespace: String,
    pub last_pos: SourceSpan,
    pub stack: Vec<usize>,
}

impl_lexer!(Lexer);

impl Lexer {
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
}

lexer!((false): top_level => [
    read_import,
    read_func,
    read_init_block,
    read_tick_block,
    read_objective,
    read_const
]);

lexer!((false): body => [
    read_const,
    read_var,
    read_call,
    read_for_loop,
    read_cond,
    read_return,
    read_assign,
    read_binop_val
]);

lexer!((true): value => [
    read_binop,
    read_value_nb
]);

lexer!((true): value_nb => [
    read_binop_val,
    read_value_nbv
]);

lexer!((true): value_nbv => [
    read_literal,
    read_array,
    read_call,
    read_unop,
    read_special,
    read_ident_full
]);
