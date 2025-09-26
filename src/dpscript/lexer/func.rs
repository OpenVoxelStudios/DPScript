use crate::{
    dpscript::{
        ast::{
            func::{FuncFlags, FunctionNode},
            node::Node,
        },
        lexer::{Lexer, Result},
        tokenizer::Token,
    },
    util::Identifier,
};

impl Lexer {
    pub fn read_func(&mut self) -> Result<Node> {
        debug!("Attempting to read function...");

        self.push();

        // TODO: Use
        let _attr = self.read_attr();

        // Flags
        // TODO: Make the modifiers not required to be in this order
        let is_pub = self.if_next_and_eat(Token::Pub);
        let is_facade = self.if_next_and_eat(Token::Facade);

        let span = self.start_parse(Token::Fn)?;
        let name = self.eat_id()?;

        self.expect(Token::LeftParen)?;

        let _args_tkns = self.eat_block(Token::LeftParen, Token::RightParen);

        // TODO: return type

        // TODO
        let mut _body = None;

        if is_facade {
            self.expect(Token::Semi)?;
        } else {
            self.expect(Token::LeftBrace)?;

            _body = Some(self.eat_block(Token::LeftBrace, Token::RightBrace));
        }

        let mut flags = FuncFlags::empty();

        if is_pub {
            flags = flags | FuncFlags::Public;
        }

        if is_facade {
            flags = flags | FuncFlags::Facade;
        }

        self.pop_in_place()?;

        debug!("Successfully read function!");

        Ok(Node::Function(FunctionNode {
            name: name.clone(),
            span,
            args: vec![],
            body: vec![],
            flags,
            ident: Identifier {
                namespace: "TODO".into(),
                path: name,
            },
            return_type: None,
        }))
    }
}
