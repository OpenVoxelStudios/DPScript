use std::str::FromStr;

use crate::dpscript::{
    lexer::{Result, err::LexerErr, util::LexerMethods},
    tokenizer::Token,
    ty::{BuiltInType, TypeRef},
};

pub trait TypeLexer: LexerMethods {
    fn read_ty(&mut self) -> Result<TypeRef> {
        let is_arr = self.if_next_and_eat(Token::LeftBracket);

        if is_arr {
            let inner = self.read_ty()?;

            let has_size = self.if_next_and_eat(Token::Semi);

            if has_size {
                let size = self.eat_uint()?;

                self.expect(Token::RightBracket)?;

                Ok(TypeRef::SizedArray(
                    Box::new(inner),
                    (size.0 as usize, size.1),
                ))
            } else {
                self.expect(Token::RightBracket)?;

                Ok(TypeRef::Array(vec![Some(inner)]))
            }
        } else {
            let (id, span) = self.eat_id()?;

            match BuiltInType::from_str(&id) {
                Ok(it) => Ok(TypeRef::BuiltIn(it)),
                Err(_) => Err(LexerErr::UnknownType { span, ty: id }),
            }
        }
    }
}

impl<T: LexerMethods> TypeLexer for T {}
