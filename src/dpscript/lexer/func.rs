use flexstr::SharedStr;

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

const FN_MODIFIERS: &[Token] = &[
    Token::Pub,
    Token::Facade,
    Token::Compiler,
    Token::Inline,
    Token::Operator,
];

impl Lexer {
    pub fn read_func(&mut self) -> Result<Node> {
        debug!("[{}] Attempting to read function...", self.nesting);

        self.push();

        let attrs = self.read_attrs()?;
        let mut flags = FuncFlags::empty();
        let mut span = None;

        while self.peek(0).is_some_and(|it| FN_MODIFIERS.contains(&it.0)) {
            let (it, sp) = self.eat_res()?;

            if span.is_none() {
                span = Some(sp);
            }

            flags |= match it {
                Token::Pub => FuncFlags::Public,
                Token::Facade => FuncFlags::Facade,
                Token::Compiler => FuncFlags::Compiler,
                Token::Inline => FuncFlags::Inline,
                Token::Operator => FuncFlags::Operator,

                _ => unreachable!("How did this happen? This is a compiler bug! Please report it!"),
            };
        }

        let fn_span = self.start_parse(Token::Fn)?;
        let mut span = span.unwrap_or(fn_span);
        let (mut name, _) = self.eat_id()?;
        let mut receiver = None;

        if self.if_next_and_eat(Token::DoubleColon) {
            let (real_name, _) = self.eat_id()?;

            receiver = Some(name);
            name = real_name;
        }

        self.expect(Token::LeftParen)?;

        let mut args = Vec::new();
        let storage: SharedStr = format!("{}:__dps/gen/funcs/{}/args", self.namespace, name).into();

        loop {
            if self.if_next_and_eat(Token::RightParen) {
                break;
            }

            if self.if_next_and_eat(Token::Comma) {
                continue;
            }

            let attrs = self.read_attrs()?;
            let is_ref = self.if_next_and_eat(Token::Ref);
            let name = self.eat_id()?;
            let is_this = attrs.contains_key("this");

            self.expect(Token::Colon)?;

            let ty = self.read_ty()?;

            if self.peek(0).is_none_or(|it| it.0 != Token::RightParen) {
                self.expect(Token::Comma)?;
            }

            args.push(FunctionArg {
                is_this,
                attrs,
                location: DataLocation {
                    path: name.0.clone(),
                    storage: storage.clone(),
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

        if flags.contains(FuncFlags::Compiler) || flags.contains(FuncFlags::Facade) {
            self.expect(Token::Semi)?;
        } else {
            self.push_func(name.clone());
            self.expect(Token::LeftBrace)?;

            while !self.if_next_and_eat_span(Token::RightBrace, &mut span) {
                body.push(self.read_body()?);
            }

            self.pop_func()?;
        }

        self.pop_in_place()?;

        debug!("[{}] Successfully read function!", self.nesting);

        let id = attrs
            .get("name")
            .map(|it| {
                it.values
                    .first()
                    .map(|it| it.as_literal().map(|it| it.as_string()))
            })
            .flatten()
            .flatten()
            .flatten()
            .unwrap_or(format!("zzz/{}/funcs/{}", self.module, name).into());

        Ok(Node::Function(FunctionNode {
            name,
            span,
            args,
            body,
            flags,
            receiver,
            attrs,
            keep: self.keep,
            scope: None,
            ident: Identifier {
                namespace: self.namespace.clone(),
                path: id,
            },
            return_type: ret,
        }))
    }
}
