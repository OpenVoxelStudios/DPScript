use crate::{
    chain_parsers,
    dpscript::{
        ast::node::Node,
        lexer::{Result, err::LexerErr, util::LexerMethods},
        tokenizer::Token,
    },
    impl_lexer,
    util::Spanned,
};
use miette::{SourceOffset, SourceSpan};

macro_rules! lexer {
    (($value: expr, $body: expr): $name: ident => [$($func: ident),*]) => {
        concat_idents::concat_idents!(name = read_, $name {
            impl Lexer {
                pub fn name(&mut self) -> Result<Node> {
                    let res = chain_parsers!(($value): self; [
                        $($func),*
                    ])?;

                    self.last.push(res.clone());

                    if $body && self.peek(0).is_some_and(|it| it.0 == Token::Semi) {
                        self.eat();
                    }

                    return Ok(res);
                }
            }
        });

        concat_idents::concat_idents!(name = read_, $name, _sep {
            impl Lexer {
                pub fn name(&mut self, sep: &Token) -> Result<Option<Node>> {
                    let res = chain_parsers!(sep, ($value): self; [
                        $($func),*
                    ])?;

                    if let Some(it) = &res {
                        self.last.push(it.clone());
                    }

                    if $body && self.peek(0).is_some_and(|it| it.0 == Token::Semi) {
                        self.eat();
                    }

                    Ok(res)
                }
            }
        });

        concat_idents::concat_idents!(name = parse_, $name {
            impl Lexer {
                pub fn name(mut self) -> Result<Vec<Node>> {
                    while self.has_next() {
                        concat_idents::concat_idents!(func = read_, $name {
                            let node = self.func()?;

                            self.nodes.push(node);
                            self.last.pop();
                        });
                    }

                    Ok(self.nodes)
                }
            }
        });

        concat_idents::concat_idents!(name = parse_, $name, _sep {
            impl Lexer {
                pub fn name(mut self, sep: Token) -> Result<Vec<Node>> {
                    while self.has_next() {
                        concat_idents::concat_idents!(func = read_, $name, _sep {
                            if let Some(node) = self.func(&sep)? {
                                self.nodes.push(node);
                            }

                            self.last.pop();
                        });
                    }

                    Ok(self.nodes)
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
    pub last: Vec<Node>,
    pub nodes: Vec<Node>,
    pub nesting: usize,
    pub function: Vec<String>,
    pub block: Vec<usize>,
    pub module: String,
    pub event_block: usize,
    pub keep: bool,
}

impl_lexer!(Lexer);

pub fn fast_ident(inp: &str) -> String {
    let mut s = String::new();

    for ch in inp.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            s.push(ch);
        } else {
            s.push('_');
        }
    }

    s.into()
}

impl Lexer {
    pub fn new(
        namespace: String,
        module: String,
        keep: bool,
        tokens: Vec<Spanned<Token>>,
    ) -> Self {
        let last = tokens
            .first()
            .map(|it| it.1.clone())
            .unwrap_or(SourceSpan::new(SourceOffset::from_location("", 0, 0), 0));

        Self {
            namespace,
            module: fast_ident(&module),
            tokens,
            pos: 0,
            last_pos: last,
            stack: Vec::new(),
            last: Vec::new(),
            nodes: Vec::new(),
            nesting: 0,
            function: Vec::new(),
            block: Vec::new(),
            event_block: 0,
            keep,
        }
    }

    pub fn func(&self) -> Result<String> {
        self.function
            .last()
            .cloned()
            .ok_or(LexerErr::IncompleteContext {
                span: self.loc(),
                cause: "func()",
            })
    }

    pub fn push_func(&mut self, func: String) {
        self.function.push(func);
        self.block.push(0);
    }

    pub fn pop_func(&mut self) -> Result<()> {
        self.function.pop().ok_or(LexerErr::IncompleteContext {
            span: self.loc(),
            cause: "pop_func() -> function",
        })?;

        self.block.pop().ok_or(LexerErr::IncompleteContext {
            span: self.loc(),
            cause: "pop_func() -> block",
        })?;

        Ok(())
    }

    pub fn block(&self) -> Result<usize> {
        self.block
            .last()
            .copied()
            .ok_or(LexerErr::IncompleteContext {
                span: self.loc(),
                cause: "block()",
            })
    }

    pub fn inc_block(&mut self) -> Result<usize> {
        let span = self.loc();

        let block = self.block.last_mut().ok_or(LexerErr::IncompleteContext {
            span,
            cause: "inc_block()",
        })?;

        *block += 1;

        Ok(*block)
    }
}

lexer!((false, false): top_level => [
    read_import,
    read_func,
    read_init_block,
    read_tick_block,
    read_objective,
    read_const,
    read_field
]);

lexer!((false, true): body => [
    read_call,
    read_const,
    read_var,
    read_for_loop,
    read_while_loop,
    read_at,
    read_cond,
    read_return,
    read_assign
]);

impl Lexer {
    pub fn read_value(&mut self) -> Result<Node> {
        let mut res = chain_parsers!((true): self; [
            read_array,
            read_binop_val,
            read_binop,
            read_binop_cond,
            read_literal,
            read_nbt,
            read_call,
            read_unop,
            read_special,
            read_ident_full
        ])?;

        while res.maybe_has_value()
            && self
                .peek(0)
                .is_some_and(|it| it.0.is_binop_val(self.peek(1)))
        {
            self.last.push(res.clone());

            if let Ok(node) = self.read_binop() {
                res = node;
            } else if let Ok(node) = self.read_binop_val() {
                res = node;
            } else if let Ok(node) = self.read_binop_cond() {
                res = node;
            } else {
                return Err(LexerErr::Unexpected {
                    span: self.loc(),
                    tkn: self.eat().unwrap().0,
                });
            }
        }

        Ok(res)
    }
}

impl Lexer {
    pub fn read_value_sep(&mut self, sep: &Token) -> Result<Option<Node>> {
        let mut res = chain_parsers!(sep,(true): self; [
            read_array,
            read_binop_val,
            read_binop,
            read_binop_cond,
            read_literal,
            read_nbt,
            read_call,
            read_unop,
            read_special,
            read_ident_full
        ])?;

        if let Some(res) = &mut res {
            while res.maybe_has_value()
                && self
                    .peek(0)
                    .is_some_and(|it| it.0.is_binop_val(self.peek(1)))
            {
                self.last.push(res.clone());

                if let Ok(node) = self.read_binop() {
                    *res = node;
                } else if let Ok(node) = self.read_binop_val() {
                    *res = node;
                } else if let Ok(node) = self.read_binop_cond() {
                    *res = node;
                } else {
                    return Err(LexerErr::Unexpected {
                        span: self.loc(),
                        tkn: self.eat().unwrap().0,
                    });
                }
            }
        }

        Ok(res)
    }
}

impl Lexer {
    pub fn parse_value(mut self) -> Result<Vec<Node>> {
        while self.has_next() {
            let node = self.read_value()?;

            self.nodes.push(node);
        }

        Ok(self.nodes)
    }
}

impl Lexer {
    pub fn parse_value_sep(mut self, sep: Token) -> Result<Vec<Node>> {
        while self.has_next() {
            if let Some(node) = self.read_value_sep(&sep)? {
                self.nodes.push(node);
            }
        }

        Ok(self.nodes)
    }
}
