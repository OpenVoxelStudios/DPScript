use crate::err::Error;
use dpscript_core::{SourceSpan, Spanned};
use dpscript_parser::Token;
use std::{fmt, marker::PhantomData};

#[derive(Default)]
pub struct ParseCx<'a> {
    _data: PhantomData<&'a ()>,
}

impl<'a> fmt::Debug for ParseCx<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParseCx").finish()
    }
}

impl<'a> ParseCx<'a> {
    pub fn unexpected(&self, tkn: Spanned<Token<'a>>) -> Error {
        Error::UnexpectedToken {
            token: tkn.0.into(),
            span: tkn.1.into(),
        }
    }

    pub fn eof(&self, span: SourceSpan) -> Error {
        Error::Eof { span: span.into() }
    }

    pub fn skip(&self) -> Error {
        Error::Skip
    }
}
