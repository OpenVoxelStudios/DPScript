use crate::{
    dpscript::{
        ast::{
            func::{FuncFlags, FunctionArg, FunctionNode},
            node::Node,
        },
        lexer::{Result, parser::Lexer, ty::TypeLexer, util::LexerMethods},
        tokenizer::Token,
        ty::{BuiltInType, TypeRef},
    },
    util::{DataLocation, Identifier},
};

impl Lexer {
    pub fn read_func(&mut self) -> Result<Node> {
        debug!("[{}] Attempting to read function...", self.nesting);

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

        let mut args = Vec::new();

        loop {
            if self.if_next_and_eat(Token::RightParen) {
                break;
            }

            if self.if_next_and_eat(Token::Comma) {
                continue;
            }

            // TODO: Use this
            let _arg_attr = self.read_attrs();

            let is_ref = self.if_next_and_eat(Token::Ref);
            let name = self.eat_id()?;

            self.expect(Token::Colon)?;

            let ty = self.read_ty()?;

            if self.peek(0).is_none_or(|it| it.0 != Token::RightParen) {
                self.expect(Token::Comma)?;
            }

            args.push(FunctionArg {
                is_this: false,
                location: DataLocation {
                    path: name.0.clone(),
                    storage: "TODO".into(),
                },
                name: name.0,
                span: name.1,
                ty,
                is_ref,
            });
        }

        let ret = if self.if_next_and_eat(Token::Minus) {
            self.expect(Token::RightAngle)?;
            self.read_ty()?
        } else {
            TypeRef::BuiltIn(BuiltInType::Void)
        };

        let mut body = Vec::new();

        if is_facade || is_compiler {
            self.expect(Token::Semi)?;
        } else {
            self.expect(Token::LeftBrace)?;

            let (tokens, _) = self.eat_block(Token::LeftBrace, Token::RightBrace);
            let parser = Lexer::new(self.namespace.clone(), tokens);

            body = parser.parse_body()?;
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

        debug!("[{}] Successfully read function!", self.nesting);

        Ok(Node::Function(FunctionNode {
            name: name.clone(),
            span,
            args,
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
