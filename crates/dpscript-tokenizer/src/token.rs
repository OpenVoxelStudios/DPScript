use crate::kw::Keyword;
use derive_more::Display;
use dpscript_core::{MSourceSpan, Spanned, StringCursor};
use miette::Diagnostic;
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize)]
pub enum BraceType {
    #[display("()")]
    Parens, // ()

    #[display("[]")]
    Brackets, // []

    #[display("{{}}")]
    Braces, // {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum Punct {
    Hash,  // #
    Semi,  // ;
    Comma, // ,
    Colon, // :
    GroupStart(BraceType),
    GroupEnd(BraceType),
}

impl fmt::Display for Punct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hash => write!(f, "#"),
            Self::Semi => write!(f, ";"),
            Self::Comma => write!(f, ","),
            Self::Colon => write!(f, ":"),
            Self::GroupStart(brace_type) => match brace_type {
                BraceType::Parens => write!(f, "("),
                BraceType::Brackets => write!(f, "["),
                BraceType::Braces => write!(f, "{{"),
            },
            Self::GroupEnd(brace_type) => match brace_type {
                BraceType::Parens => write!(f, ")"),
                BraceType::Brackets => write!(f, "]"),
                BraceType::Braces => write!(f, "}}"),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize)]
pub enum Comparison {
    #[display("==")]
    Equal, // ==
    #[display("<")]
    Less, // <
    #[display("<=")]
    LessEqual, // <=
    #[display(">")]
    Greater, // >
    #[display(">=")]
    GreaterEqual, // >=
    #[display("!=")]
    NotEqual, // !=
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize)]
pub enum Assignment {
    #[display("=")]
    Equal, // =
    #[display("+=")]
    AddEqual, // +=
    #[display("-=")]
    SubEqual, // -=
    #[display("*=")]
    MulEqual, // *=
    #[display("/=")]
    DivEqual, // /=
    #[display("%=")]
    ModEqual, // %=
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize)]
pub enum Operator {
    #[display("@")]
    At, // @
    #[display("+")]
    Plus, // +
    #[display("-")]
    Minus, // -
    #[display("*")]
    Star, // *
    #[display("/")]
    Slash, // /
    #[display("%")]
    Percent, // %
    #[display("!")]
    Bang, // !
    #[display(".")]
    Dot, // .
    #[display("..")]
    Range, // ..
    #[display("::")]
    ModulePath, // ::
    #[display("->")]
    Returns, // ->
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize)]
pub enum Literal<'a> {
    #[display("{_0}")]
    Identifier(&'a str),

    #[display("\"{_0}\"")]
    String(&'a str),

    #[display("{_0}")]
    Int(i32),

    #[display("{_0}")]
    Long(i64),

    #[display("{_0}")]
    Byte(i8), // Minecraft bytes are signed (java moment)

    #[display("{_0}")]
    Float(OrderedFloat<f32>),

    #[display("{_0}")]
    Double(OrderedFloat<f64>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize)]
pub enum OwnedLiteral {
    #[display("{_0}")]
    Identifier(String),

    #[display("\"{_0}\"")]
    String(String),

    #[display("{_0}")]
    Int(i32),

    #[display("{_0}")]
    Long(i64),

    #[display("{_0}")]
    Byte(i8), // Minecraft bytes are signed (java moment)

    #[display("{_0}")]
    Float(OrderedFloat<f32>),

    #[display("{_0}")]
    Double(OrderedFloat<f64>),
}

impl<'a> From<Literal<'a>> for OwnedLiteral {
    fn from(value: Literal<'a>) -> Self {
        match value {
            Literal::Identifier(it) => OwnedLiteral::Identifier(it.into()),
            Literal::String(it) => OwnedLiteral::String(it.into()),
            Literal::Int(it) => OwnedLiteral::Int(it),
            Literal::Long(it) => OwnedLiteral::Long(it),
            Literal::Byte(it) => OwnedLiteral::Byte(it),
            Literal::Float(it) => OwnedLiteral::Float(it),
            Literal::Double(it) => OwnedLiteral::Double(it),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Serialize)]
pub enum Token<'a> {
    #[display("{_0}")]
    Keyword(Keyword),

    #[display("{_0}")]
    Punct(Punct),

    #[display("{_0}")]
    Comparison(Comparison),

    #[display("{_0}")]
    Assignment(Assignment),

    #[display("{_0}")]
    Operator(Operator),

    #[display("{_0}")]
    Literal(Literal<'a>),
}

#[derive(Debug, Error, Diagnostic)]
pub enum TokenError {
    #[error("unexpected character: {ch:?}")]
    #[diagnostic(code(dpscript_tokenizer::unexpected))]
    Unexpected {
        ch: char,

        #[label("here")]
        span: MSourceSpan,
    },

    #[error("float/double cannot have more than one decimal")]
    DoubleDotFloat,

    #[error(transparent)]
    ParseInt {
        #[from]
        inner: ParseIntError,
    },

    #[error(transparent)]
    ParseFloat {
        #[from]
        inner: ParseFloatError,
    },
}

macro_rules! one_extra {
    ($iter: ident; $ch: expr, $g1: ident::$v1: ident, $g2: ident::$v2: ident) => {
        if $iter.next_if_eq($ch).is_some() {
            (Token::$g1($g1::$v1), $iter.end_span())
        } else {
            (Token::$g2($g2::$v2), $iter.end_span())
        }
    };
}

macro_rules! two_extra {
    ($iter: ident; $ch: expr, $g1: ident::$v1: ident, $ch2: expr, $g2: ident::$v2: ident, $g3: ident::$v3: ident) => {
        if $iter.next_if_eq($ch).is_some() {
            (Token::$g1($g1::$v1), $iter.end_span())
        } else {
            $iter.clear_peeker();

            if $iter.next_if_eq($ch2).is_some() {
                (Token::$g2($g2::$v2), $iter.end_span())
            } else {
                (Token::$g2($g3::$v3), $iter.end_span())
            }
        }
    };
}

macro_rules! token {
    ($iter: ident; $group: ident::$name: ident) => {
        (Token::$group($group::$name), $iter.end_span())
    };

    ($iter: ident; $group: ident::$name: ident($($tkn: tt)+)) => {
        (Token::$group($group::$name($($tkn)+)), $iter.end_span())
    };

    ($iter: ident; Punct::GroupStart = $name: ident) => {
        (Token::Punct(Punct::GroupStart(BraceType::$name)), $iter.end_span())
    };

    ($iter: ident; Punct::GroupEnd = $name: ident) => {
        (Token::Punct(Punct::GroupEnd(BraceType::$name)), $iter.end_span())
    };
}

#[track_caller]
pub fn parse_next<'a>(
    iter: &mut StringCursor<'a>,
) -> Option<Result<Option<Spanned<Token<'a>>>, TokenError>> {
    let _ = iter.take_while(|it| it.is_whitespace());

    iter.begin_span();

    Some(Ok(Some(match iter.next()? {
        c if c.is_whitespace() => return Some(Ok(None)),

        '#' => token!(iter; Punct::Hash),
        ';' => token!(iter; Punct::Semi),
        ',' => token!(iter; Punct::Comma),
        '@' => token!(iter; Operator::At),

        '(' => token!(iter; Punct::GroupStart = Parens),
        ')' => token!(iter; Punct::GroupEnd = Parens),

        '[' => token!(iter; Punct::GroupStart = Brackets),
        ']' => token!(iter; Punct::GroupEnd = Brackets),

        '{' => token!(iter; Punct::GroupStart = Braces),
        '}' => token!(iter; Punct::GroupEnd = Braces),

        ':' => one_extra!(iter; ':', Operator::ModulePath, Punct::Colon),
        '=' => one_extra!(iter; '=', Comparison::Equal, Assignment::Equal),
        '!' => one_extra!(iter; '=', Comparison::NotEqual, Operator::Bang),
        '<' => one_extra!(iter; '=', Comparison::LessEqual, Comparison::Less),
        '>' => one_extra!(iter; '=', Comparison::GreaterEqual, Comparison::Greater),
        '+' => one_extra!(iter; '=', Assignment::AddEqual, Operator::Plus),
        '-' => two_extra!(iter; '=', Assignment::SubEqual, '>', Operator::Returns, Operator::Minus),
        '*' => one_extra!(iter; '=', Assignment::MulEqual, Operator::Star),
        '%' => one_extra!(iter; '=', Assignment::ModEqual, Operator::Percent),
        '.' => one_extra!(iter; '.', Operator::Range, Operator::Dot),

        '/' => {
            if iter.next_if_eq('/').is_some() {
                let _ = iter.take_while(|it| it != '\n');
                iter.next();

                return Some(Ok(None));
            } else if iter.next_if_eq('=').is_some() {
                token!(iter; Assignment::DivEqual)
            } else {
                token!(iter; Operator::Slash)
            }
        }

        '"' => {
            let mut last = '"';
            let mut len = 0;

            while let Some(ch) = iter.peek() {
                if ch == '"' && last != '\\' {
                    break;
                } else if ch == '"' && last == '\\' {
                    last = '"';
                } else {
                    len += 1;
                    last = ch;
                }
            }

            let content = iter.take(len);

            iter.next();

            (Token::Literal(Literal::String(content)), iter.end_span())
        }

        c if c.is_alphabetic()
            && let Some(kw) = Keyword::try_parse(c, iter) =>
        {
            (Token::Keyword(kw), iter.end_span())
        }

        c if c.is_alphabetic() || c == '_' => {
            let mut len = 0;

            while iter
                .peek()
                .is_some_and(|it| it.is_alphanumeric() || it == '_')
            {
                len += 1;
            }

            iter.back();

            (
                Token::Literal(Literal::Identifier(iter.take(len + 1))),
                iter.end_span(),
            )
        }

        c if c.is_numeric() => {
            let mut dot = false;
            let mut len = 0;

            while let Some(ch) = iter.peek() {
                if !ch.is_numeric() || ch != '.' {
                    break;
                }

                if ch == '.' {
                    if dot {
                        return Some(Err(TokenError::DoubleDotFloat));
                    } else {
                        dot = true;
                        len += 1;
                    }
                } else {
                    len += 1;
                }
            }

            iter.back();

            let buf = iter.take(len + 1);

            match iter.next_if(|it| ['d', 'f', 'b', 'L'].contains(&it)) {
                Some('d') => match buf.parse::<f64>() {
                    Ok(it) => token!(iter; Literal::Double(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },

                Some('f') => match buf.parse::<f32>() {
                    Ok(it) => token!(iter; Literal::Float(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },

                Some('b') => match buf.parse::<i8>() {
                    Ok(it) => token!(iter; Literal::Byte(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },

                Some('L') => match buf.parse::<i64>() {
                    Ok(it) => token!(iter; Literal::Long(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },

                Some(c) => {
                    return Some(Err(TokenError::Unexpected {
                        ch: c,
                        span: iter.end_span().into(),
                    }));
                } // how does this even happen??

                None => match buf.parse::<i32>() {
                    Ok(it) => token!(iter; Literal::Int(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },
            }
        }

        c => {
            return Some(Err(TokenError::Unexpected {
                ch: c,
                span: iter.end_span().into(),
            }));
        }
    })))
}
