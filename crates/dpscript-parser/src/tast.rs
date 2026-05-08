//! T-AST: Token Abstract Syntax Tree
//!
//! A generic version of an AST, which essentially just differentiates between groups and tokens.

use std::fmt;

use derive_more::Display;
use dpscript_core::{CopyCursor, DynArray, MSourceSpan, Spanned};
use dpscript_tokenizer::token::{Punct as TPunct, Token as TToken};
pub use dpscript_tokenizer::{
    kw::Keyword,
    token::{Assignment, BraceType, Comparison, Literal, Operator},
};
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("a delimiter group was left unclosed")]
    #[diagnostic(code(dpscript_parser::unclosed_group))]
    UnclosedGroup {
        #[label("here")]
        span: MSourceSpan,
    },

    #[error("there is no delimiter group to close")]
    #[diagnostic(code(dpscript_parser::unexpected_group_close))]
    UnexpectedGroupClose {
        #[label("here")]
        span: MSourceSpan,
    },

    #[error("mismatched brace type: expected '{expect}', but got: '{got}")]
    #[diagnostic(code(dpscript_parser::mismatched_brace_type))]
    MismatchedBraceType {
        expect: BraceType,
        got: BraceType,

        #[label("here")]
        span: MSourceSpan,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum Punct {
    #[display("#")]
    Hash, // #
    #[display(";")]
    Semi, // ;
    #[display(",")]
    Comma, // ,
    #[display(":")]
    Colon, // :
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token<'a> {
    Keyword(Keyword),
    Punct(Punct),
    Comparison(Comparison),
    Assignment(Assignment),
    Operator(Operator),
    Literal(Literal<'a>),
    BraceGroup(BraceType, DynArray<Spanned<Token<'a>>>),
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyword(keyword) => write!(f, "{keyword}"),
            Self::Punct(punct) => write!(f, "{punct}"),
            Self::Comparison(comparison) => write!(f, "{comparison}"),
            Self::Assignment(assignment) => write!(f, "{assignment}"),
            Self::Operator(operator) => write!(f, "{operator}"),
            Self::Literal(literal) => write!(f, "{literal}"),

            Self::BraceGroup(brace_type, tokens) => {
                match brace_type {
                    BraceType::Parens => write!(f, "("),
                    BraceType::Brackets => write!(f, "["),
                    BraceType::Braces => write!(f, "{{"),
                }?;

                for tkn in tokens {
                    write!(f, "{}", tkn.0)?;
                }

                match brace_type {
                    BraceType::Parens => write!(f, ")"),
                    BraceType::Brackets => write!(f, "]"),
                    BraceType::Braces => write!(f, "}}"),
                }
            }
        }
    }
}

pub fn tast_from_tokens<'a>(
    tokens: Vec<Spanned<TToken<'a>>>,
) -> Result<Vec<Spanned<Token<'a>>>, Error> {
    let mut cursor = CopyCursor::new(tokens);
    let mut stack: Vec<Vec<Spanned<Token<'a>>>> = Vec::new();
    let mut braces = Vec::new();

    stack.push(Vec::new());

    while let Some((tkn, span)) = cursor.next() {
        match tkn {
            TToken::Punct(TPunct::GroupStart(it)) => {
                braces.push(it);
                stack.push(Vec::new());
            }

            TToken::Punct(TPunct::GroupEnd(it)) => match braces.last() {
                Some(last) => {
                    if *last != it {
                        return Err(Error::MismatchedBraceType {
                            expect: *last,
                            got: it,
                            span: span.into(),
                        });
                    } else {
                        let iter = stack.pop().unwrap();
                        let kind = braces.pop().unwrap();

                        let span = if iter.is_empty() {
                            span
                        } else {
                            iter.first().unwrap().1 + iter.last().unwrap().1
                        };

                        stack.last_mut().unwrap().push((
                            Token::BraceGroup(kind, DynArray::from_array(iter.into_boxed_slice())),
                            span,
                        ));
                    }
                }

                None => {
                    return Err(Error::UnexpectedGroupClose { span: span.into() });
                }
            },

            TToken::Punct(TPunct::Colon) => stack
                .last_mut()
                .unwrap()
                .push((Token::Punct(Punct::Colon), span)),

            TToken::Punct(TPunct::Comma) => stack
                .last_mut()
                .unwrap()
                .push((Token::Punct(Punct::Comma), span)),

            TToken::Punct(TPunct::Hash) => stack
                .last_mut()
                .unwrap()
                .push((Token::Punct(Punct::Hash), span)),

            TToken::Punct(TPunct::Semi) => stack
                .last_mut()
                .unwrap()
                .push((Token::Punct(Punct::Semi), span)),

            TToken::Assignment(it) => stack
                .last_mut()
                .unwrap()
                .push((Token::Assignment(it), span)),

            TToken::Comparison(it) => stack
                .last_mut()
                .unwrap()
                .push((Token::Comparison(it), span)),

            TToken::Keyword(it) => stack.last_mut().unwrap().push((Token::Keyword(it), span)),
            TToken::Literal(it) => stack.last_mut().unwrap().push((Token::Literal(it), span)),
            TToken::Operator(it) => stack.last_mut().unwrap().push((Token::Operator(it), span)),
        }
    }

    Ok(stack.pop().unwrap())
}
