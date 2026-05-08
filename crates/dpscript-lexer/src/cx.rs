use crate::err::Error;
use dpscript_core::{SourceSpan, Spanned};
use dpscript_parser::Token;
use std::marker::PhantomData;

#[derive(Default)]
pub struct ParseCx<'a> {
    _data: PhantomData<&'a ()>,
}

impl<'a> ParseCx<'a> {
    pub fn unexpected(&self, tkn: Spanned<Token<'a>>) -> Error<'a> {
        Error::UnexpectedToken {
            token: tkn.0,
            span: tkn.1.into(),
        }
    }

    pub fn eof(&self, span: SourceSpan) -> Error<'a> {
        Error::Eof { span: span.into() }
    }
}
