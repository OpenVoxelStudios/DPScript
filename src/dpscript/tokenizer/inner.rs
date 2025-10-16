use super::{Designator, Token, Tokenizer};
use crate::{
    Result,
    error::TokenizerError,
    util::{IsNotIdent, Spanned},
};

impl Tokenizer {
    pub(super) fn tokenize_inner(&mut self) -> Result<()> {
        let Some(ch) = self.cursor.peek() else {
            // File is empty.
            // Yay, guys, we did it! It was *sooooooooooooo* hard...
            return Ok(());
        };

        if ch.is_whitespace() {
            self.cursor.skip(1);
            return Ok(());
        }

        if ch == '/' && self.cursor.peek_ahead(1).is_some_and(|v| v == '/') {
            while let Some(ch) = self.cursor.next() {
                if ch == '\n' {
                    break;
                }
            }

            return Ok(());
        }

        let basic = match ch {
            ',' => self.skip_1(Token::Comma),
            '[' => self.skip_1(Token::LeftBracket),
            ']' => self.skip_1(Token::RightBracket),
            '{' => self.skip_1(Token::LeftBrace),
            '}' => self.skip_1(Token::RightBrace),
            '(' => self.skip_1(Token::LeftParen),
            ')' => self.skip_1(Token::RightParen),
            '<' => self.skip_1(Token::LeftAngle),
            '>' => self.skip_1(Token::RightAngle),
            ';' => self.skip_1(Token::Semi),
            '=' => self.skip_1(Token::Equal),
            '-' => self.skip_1(Token::Minus),
            '+' => self.skip_1(Token::Plus),
            '*' => self.skip_1(Token::Star),
            '&' => self.skip_1(Token::And),
            '#' => self.skip_1(Token::Hash),
            '!' => self.skip_1(Token::Exclamation),
            '~' => self.skip_1(Token::Tilde),
            '/' => self.skip_1(Token::Slash),
            '%' => self.skip_1(Token::Modulo),
            '|' => self.skip_1(Token::Or),
            '^' => self.skip_1(Token::Xor),

            ':' => {
                if self.cursor.peek_ahead(1).is_some_and(|v| v == ':') {
                    let span = self.cursor.span(2);
                    self.cursor.skip(2);
                    Some((Token::DoubleColon, span))
                } else {
                    self.skip_1(Token::Colon)
                }
            }

            '.' => {
                if self.cursor.peek_ahead(1).is_some_and(|v| v == '.')
                    && self.cursor.peek_ahead(2).is_some_and(|v| v == '.')
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(3);
                    Some((Token::Ellipsis, span))
                } else if self.cursor.peek_ahead(1).is_some_and(|v| v == '.') {
                    let span = self.cursor.span(2);
                    self.cursor.skip(2);
                    Some((Token::Range, span))
                } else {
                    self.skip_1(Token::Dot)
                }
            }

            '"' => {
                let mut buf = Vec::new();

                self.cursor.skip(1);

                while let Some(ch) = self.cursor.next() {
                    if ch == '\\' && self.cursor.peek().is_some_and(|v| v == '"') {
                        self.cursor.skip(1);
                        buf.push('"');
                        continue;
                    }

                    if ch == '"' {
                        break;
                    } else {
                        buf.push(ch);
                    }
                }

                let len = buf.len();

                Some((
                    Token::String(String::from_iter(buf).into()),
                    self.cursor.span_prev(len, len),
                ))
            }

            other => {
                if other.is_ascii_digit() {
                    self.cursor.skip(1);

                    let mut buf = Vec::new();

                    buf.push(ch);

                    while let Some(ch) = self.cursor.peek() {
                        if ch.is_ascii_digit() || (ch == '.' && !buf.contains(&'.')) {
                            buf.push(ch);
                            self.cursor.skip(1);
                        } else {
                            break;
                        }
                    }

                    let designator = match self.cursor.peek() {
                        Some('d') => Some(Designator::Double),
                        Some('f') => Some(Designator::Float),
                        Some('b') => Some(Designator::Bool),
                        Some(other) => {
                            if other.is_alphabetic() {
                                return Err(TokenizerError {
                                    src: self.cursor.source(),
                                    at: self.cursor.span(1),
                                    err: format!("Unknown suffix for literal number: {other}"),
                                }
                                .into());
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    let span = match designator {
                        Some(_) => {
                            self.cursor.skip(1);
                            self.cursor.span_prev(buf.len() + 1, buf.len() + 1)
                        }

                        _ => self.cursor.span_prev(buf.len(), buf.len()),
                    };

                    if buf.contains(&'.') {
                        if let Ok(it) = buf.iter().collect::<String>().parse() {
                            match designator {
                                Some(Designator::Bool) => match it {
                                    0.0 => Some((Token::Bool(false), span)),
                                    1.0 => Some((Token::Bool(true), span)),
                                    _ => {
                                        return Err(TokenizerError {
                                            src: self.cursor.source(),
                                            at: span,
                                            err: format!("Invalid value for boolean literal: {it}"),
                                        }
                                        .into());
                                    }
                                },
                                Some(Designator::Float) => Some((Token::Float(it as f32), span)),
                                _ => Some((Token::Double(it), span)),
                            }
                        } else {
                            return Err(TokenizerError {
                                src: self.cursor.source(),
                                at: span,
                                err: format!(
                                    "Could not parse a float or double: {}",
                                    buf.iter().collect::<String>()
                                ),
                            }
                            .into());
                        }
                    } else {
                        if let Ok(it) = buf.iter().collect::<String>().parse() {
                            match designator {
                                Some(Designator::Bool) => match it {
                                    0 => Some((Token::Bool(false), span)),
                                    1 => Some((Token::Bool(true), span)),
                                    _ => {
                                        return Err(TokenizerError {
                                            src: self.cursor.source(),
                                            at: span,
                                            err: format!("Invalid value for boolean literal: {it}"),
                                        }
                                        .into());
                                    }
                                },
                                Some(Designator::Float) => Some((Token::Float(it as f32), span)),
                                Some(Designator::Double) => Some((Token::Double(it as f64), span)),
                                _ => Some((Token::Int(it), span)),
                            }
                        } else {
                            return Err(TokenizerError {
                                src: self.cursor.source(),
                                at: span,
                                err: format!(
                                    "Could not parse an int: {}",
                                    buf.iter().collect::<String>()
                                ),
                            }
                            .into());
                        }
                    }
                } else {
                    None
                }
            }
        };

        if let Some(tkn) = basic {
            self.tokens.push(tkn);
            return Ok(());
        }

        let kw = match self.cursor.next_group_spanned(|it| it.is_not_ident()) {
            Some((group, span)) => match group.as_str() {
                "if" => Some((Token::If, span)),
                "in" => Some((Token::In, span)),
                "init" => Some((Token::Init, span)),
                "inline" => Some((Token::Inline, span)),
                "import" => Some((Token::Import, span)),
                "selector" => Some((Token::Selector, span)),
                "export" => Some((Token::Export, span)),
                "enum" => Some((Token::Enum, span)),
                "else" => Some((Token::Else, span)),
                "fn" => Some((Token::Fn, span)),
                "for" => Some((Token::For, span)),
                "facade" => Some((Token::Facade, span)),
                "false" => Some((Token::Bool(false), span)),
                "pub" => Some((Token::Pub, span)),
                "pos" => Some((Token::Pos, span)),
                "const" => Some((Token::Const, span)),
                "compiler" => Some((Token::Compiler, span)),
                "component" => Some((Token::Component, span)),
                "c" => Some((Token::ComponentShort, span)),
                "let" => Some((Token::Let, span)),
                "return" => Some((Token::Return, span)),
                "ref" => Some((Token::Ref, span)),
                "objective" => Some((Token::Objective, span)),
                "module" => Some((Token::Module, span)),
                "nbt" => Some((Token::Nbt, span)),
                "tick" => Some((Token::Tick, span)),
                "true" => Some((Token::Bool(true), span)),
                "at" => Some((Token::At, span)),
                "as" => Some((Token::As, span)),
                "while" => Some((Token::While, span)),
                "operator" => Some((Token::Operator, span)),
                "instance" => Some((Token::Instance, span)),
                "field" => Some((Token::Field, span)),

                other => {
                    if other.is_empty() {
                        None
                    } else {
                        if other.chars().next().unwrap().is_numeric() {
                            None
                        } else {
                            Some((Token::Ident(other.into()), span))
                        }
                    }
                }
            },

            // FIXME: Should this be an error?
            None => return Ok(()),
        };

        if let Some(tkn) = kw {
            self.tokens.push(tkn);
            return Ok(());
        }

        Err(TokenizerError {
            src: self.cursor.source(),
            at: self.cursor.span(1),
            err: format!("Unexpected character: {}", ch),
        }
        .into())
    }

    fn skip_1(&mut self, res: Token) -> Option<Spanned<Token>> {
        self.cursor.skip(1);
        Some((res, self.cursor.span(1)))
    }
}
