use miette::SourceSpan;

use crate::{
    dpscript::{
        ast::{
            block::{BlockKind, BlockNode},
            node::Node,
        },
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_init_block(&mut self) -> Result<Node> {
        debug!("Attempting to read init block...");

        self.push();

        let span = self.start_parse(Token::Init)?;

        self.read_block_inner(span, BlockKind::Init)
    }

    pub fn read_tick_block(&mut self) -> Result<Node> {
        debug!("Attempting to read tick block...");

        self.push();

        let span = self.start_parse(Token::Tick)?;

        self.read_block_inner(span, BlockKind::Tick)
    }

    fn read_block_inner(&mut self, span: SourceSpan, kind: BlockKind) -> Result<Node> {
        self.expect(Token::LeftBrace)?;

        let (body, last) = self.eat_block(Token::LeftBrace, Token::RightBrace);
        let body = Lexer::new(self.namespace.clone(), body).parse_body()?;

        self.pop_in_place()?;

        debug!("Successfully read {kind} block!");

        Ok(Node::Block(BlockNode {
            kind,
            span: span.add(last),
            body,
        }))
    }
}
