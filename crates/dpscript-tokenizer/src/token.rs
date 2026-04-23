use std::num::{ParseFloatError, ParseIntError};

use crate::kw::Keyword;
use ordered_float::OrderedFloat;
use peekmore::PeekMoreIterator;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BraceType {
    Parens,   // ()
    Brackets, // []
    Braces,   // {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Punct {
    Hash,  // #
    Semi,  // ;
    Comma, // ,
    Colon, // :
    GroupStart(BraceType),
    GroupEnd(BraceType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Comparison {
    Equal,        // ==
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    NotEqual,     // !=
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Assignment {
    Equal,    // =
    AddEqual, // +=
    SubEqual, // -=
    MulEqual, // *=
    DivEqual, // /=
    ModEqual, // %=
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operator {
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Bang,       // !
    Dot,        // .
    Range,      // ..
    ModulePath, // ::
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    // insert praying hands emoji here
    // these HAVE to be Copy so...
    Identifier(&'static str),
    String(&'static str),

    Int(i32),
    Long(i64),
    Byte(i8), // Minecraft bytes are signed (java moment)
    Float(OrderedFloat<f32>),
    Double(OrderedFloat<f64>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    Keyword(Keyword),
    Punct(Punct),
    Comparison(Comparison),
    Assignment(Assignment),
    Operator(Operator),
    Literal(Literal),
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("unexpected character: {0}")]
    Unexpected(char),

    #[error("float/double cannot have more than one decimal")]
    DoubleDotFloat,

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),
}

macro_rules! one_extra {
    ($iter: ident; $ch: expr, $g1: ident::$v1: ident, $g2: ident::$v2: ident) => {
        if $iter.next_if_eq(&$ch).is_some() {
            Token::$g1($g1::$v1)
        } else {
            Token::$g2($g2::$v2)
        }
    };
}

pub fn parse_next<I: Iterator<Item = char>>(
    iter: &mut PeekMoreIterator<I>,
) -> Option<Result<Token, TokenError>> {
    while iter.next_if(|it| it.is_whitespace()).is_some() {}

    Some(Ok(match iter.next()? {
        '#' => Token::Punct(Punct::Hash),
        ';' => Token::Punct(Punct::Semi),
        ',' => Token::Punct(Punct::Comma),
        
        '(' => Token::Punct(Punct::GroupStart(BraceType::Parens)),
        ')' => Token::Punct(Punct::GroupEnd(BraceType::Parens)),
        
        '[' => Token::Punct(Punct::GroupStart(BraceType::Brackets)),
        ']' => Token::Punct(Punct::GroupEnd(BraceType::Brackets)),
        
        '{' => Token::Punct(Punct::GroupStart(BraceType::Braces)),
        '}' => Token::Punct(Punct::GroupEnd(BraceType::Braces)),
        
        ':' => one_extra!(iter; ':', Operator::ModulePath, Punct::Colon),
        '=' => one_extra!(iter; '=', Comparison::Equal, Assignment::Equal),
        '!' => one_extra!(iter; '=', Comparison::NotEqual, Operator::Bang),
        '<' => one_extra!(iter; '=', Comparison::LessEqual, Comparison::Less),
        '>' => one_extra!(iter; '=', Comparison::GreaterEqual, Comparison::Greater),
        '+' => one_extra!(iter; '=', Assignment::AddEqual, Operator::Plus),
        '-' => one_extra!(iter; '=', Assignment::SubEqual, Operator::Minus),
        '*' => one_extra!(iter; '=', Assignment::MulEqual, Operator::Star),
        '%' => one_extra!(iter; '=', Assignment::ModEqual, Operator::Percent),
        '.' => one_extra!(iter; '.', Operator::Range, Operator::Dot),

        '/' => {
            if iter.next_if_eq(&'/').is_some() {
                let _ = iter.take_while(|it| *it != '\n').collect::<Vec<_>>();

                return parse_next(iter);
            } else if iter.next_if_eq(&'=').is_some() {
                Token::Assignment(Assignment::DivEqual)
            } else {
                Token::Operator(Operator::Slash)
            }
        }

        '"' => {
            let mut buf = String::new();
            let mut last = '"';

            while let Some(ch) = iter.next() {
                if ch == '"' && last != '\\' {
                    buf.push(last);
                    break;
                } else if ch == '"' && last == '\\' {
                    last = '"';
                } else {
                    buf.push(last);
                    last = ch;
                }
            }

            if !buf.is_empty() {
                buf.remove(0);
            }

            // insert praying hands emoji here
            // these HAVE to be Copy so...
            let buf = Box::leak(Box::new(buf));
            let s = buf.as_str();

            Token::Literal(Literal::String(s))
        }

        c if c.is_alphanumeric()
            && let Some(kw) = Keyword::try_parse(c, iter) =>
        {
            Token::Keyword(kw)
        }

        c if c.is_alphabetic() || c == '_' => {
            let mut buf = String::new();

            buf.push(c);

            while let Some(ch) = iter.next_if(|it| it.is_alphanumeric() || *it == '_') {
                buf.push(ch);
            }

            // insert praying hands emoji here
            // these HAVE to be Copy so...
            let buf = Box::leak(Box::new(buf));
            let s = buf.as_str();

            Token::Literal(Literal::Identifier(s))
        }

        c if c.is_numeric() => {
            let mut buf = String::new();
            let mut dot = false;

            buf.push(c);

            while let Some(ch) = iter.next_if(|it| it.is_numeric() || *it == '.') {
                if ch == '.' {
                    if dot {
                        return Some(Err(TokenError::DoubleDotFloat));
                    } else {
                        dot = true;
                        buf.push(ch);
                    }
                } else {
                    buf.push(ch);
                }
            }

            match iter.next_if(|it| ['d', 'f', 'b', 'L'].contains(&it)) {
                Some('d') => match buf.parse::<f64>() {
                    Ok(it) => Token::Literal(Literal::Double(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },

                Some('f') => match buf.parse::<f32>() {
                    Ok(it) => Token::Literal(Literal::Float(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },

                Some('b') => match buf.parse::<i8>() {
                    Ok(it) => Token::Literal(Literal::Byte(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },

                Some('L') => match buf.parse::<i64>() {
                    Ok(it) => Token::Literal(Literal::Long(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },

                Some(c) => return Some(Err(TokenError::Unexpected(c))), // how does this even happen??

                None => match buf.parse::<i32>() {
                    Ok(it) => Token::Literal(Literal::Int(it.into())),
                    Err(err) => return Some(Err(err.into())),
                },
            }
        }

        c => return Some(Err(TokenError::Unexpected(c))),
    }))
}
