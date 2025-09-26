use crate::{
    dpscript::{
        ast::{
            func::{FuncFlags, FunctionNode},
            node::Node,
        },
        lexer::{parser::{BodyLexer, TopLevelLexer}, ty::TypeLexer, util::LexerMethods, Result},
        tokenizer::Token,
    },
    util::Identifier,
};

impl TopLevelLexer {
    pub fn read_func(&mut self) -> Result<Node> {
        debug!("Attempting to read function...");

        self.push();

        // TODO: Use
        let _attr = self.read_attrs();

        // Flags
        // TODO: Make the modifiers not required to be in this order
        let is_pub = self.if_next_and_eat(Token::Pub);
        let is_facade = self.if_next_and_eat(Token::Facade);
        let is_compiler = self.if_next_and_eat(Token::Compiler);

        let span = self.start_parse(Token::Fn)?;
        let (name, _) = self.eat_id()?;

        self.expect(Token::LeftParen)?;

        // TODO
        let (_args_tkns, _) = self.eat_block(Token::LeftParen, Token::RightParen);

        let has_ret_ty = self.if_next_and_eat(Token::Minus);
        let mut ret = None;

        if has_ret_ty {
            self.expect(Token::RightAngle)?;

            ret = Some(self.read_ty()?);
        }

        let mut body = Vec::new();

        if is_facade || is_compiler {
            self.expect(Token::Semi)?;
        } else {
            self.expect(Token::LeftBrace)?;

            let (tokens, _) = self.eat_block(Token::LeftBrace, Token::RightBrace);
            let parser = BodyLexer::new(self.namespace.clone(), tokens);

            body = parser.parse()?;
        }

        let mut flags = FuncFlags::empty();

        if is_pub {
            flags = flags | FuncFlags::Public;
        }

        if is_facade {
            flags = flags | FuncFlags::Facade;
        }

        if is_compiler {
            flags = flags | FuncFlags::Compiler;
        }

        self.pop_in_place()?;

        debug!("Successfully read function!");

        Ok(Node::Function(FunctionNode {
            name: name.clone(),
            span,
            args: vec![], // TODO
            body,
            flags,
            ident: Identifier {
                namespace: self.namespace.clone(),
                path: name,
            },
            return_type: ret,
        }))
    }
}
