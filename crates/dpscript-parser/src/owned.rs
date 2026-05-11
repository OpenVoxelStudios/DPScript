use crate::{Punct, Token};
use dpscript_core::{DynArray, Spanned};
use dpscript_tokenizer::{
    kw::Keyword,
    token::{Assignment, BraceType, Comparison, Operator, OwnedLiteral},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnedToken {
    Keyword(Keyword),
    Punct(Punct),
    Comparison(Comparison),
    Assignment(Assignment),
    Operator(Operator),
    Literal(OwnedLiteral),
    BraceGroup(BraceType, DynArray<Spanned<OwnedToken>>),
}

impl fmt::Display for OwnedToken {
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

impl<'a> Token<'a> {
    fn conv(self, allow_recurse: bool) -> OwnedToken {
        match self {
            Token::Keyword(it) => OwnedToken::Keyword(it),
            Token::Punct(it) => OwnedToken::Punct(it),
            Token::Comparison(it) => OwnedToken::Comparison(it),
            Token::Assignment(it) => OwnedToken::Assignment(it),
            Token::Operator(it) => OwnedToken::Operator(it),
            Token::Literal(it) => OwnedToken::Literal(it.into()),
            Token::BraceGroup(brace_type, dyn_array) => OwnedToken::BraceGroup(
                brace_type,
                if allow_recurse {
                    DynArray::from_array(
                        dyn_array
                            .into_iter()
                            .map(|it| (it.0.conv(false), it.1))
                            .collect::<Vec<_>>()
                            .into_boxed_slice(),
                    )
                } else {
                    DynArray::from_array(Box::new([]))
                },
            ),
        }
    }
}

impl<'a> From<Token<'a>> for OwnedToken {
    fn from(value: Token<'a>) -> Self {
        value.conv(true)
    }
}
