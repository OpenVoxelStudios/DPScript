use super::{Token, Tokenizer};
use crate::{Result, TokenizerError, util::IsNotIdent};

impl Tokenizer {
    pub(super) fn tokenize_inner(&mut self, ch: char) -> Result<()> {
        if ch.is_whitespace() {
            return Ok(());
        }

        if ch == '/' {
            if self.cursor.peek().is_some_and(|v| v == '/') {
                // println!("==>>> SKIPPING COMMENT");

                while let Some(ch) = self.cursor.next() {
                    if ch == '\n' {
                        break;
                    }
                }

                return Ok(());
            }
        }

        let sym = match ch {
            ':' => Some((Token::Colon, self.cursor.span(1))),
            ',' => Some((Token::Comma, self.cursor.span(1))),
            '[' => Some((Token::LeftBracket, self.cursor.span(1))),
            ']' => Some((Token::RightBracket, self.cursor.span(1))),
            '{' => Some((Token::LeftBrace, self.cursor.span(1))),
            '}' => Some((Token::RightBrace, self.cursor.span(1))),
            '(' => Some((Token::LeftParen, self.cursor.span(1))),
            ')' => Some((Token::RightParen, self.cursor.span(1))),
            '<' => Some((Token::LeftAngle, self.cursor.span(1))),
            '>' => Some((Token::RightAngle, self.cursor.span(1))),
            ';' => Some((Token::Semi, self.cursor.span(1))),
            '=' => Some((Token::Equal, self.cursor.span(1))),
            '-' => Some((Token::Minus, self.cursor.span(1))),
            '+' => Some((Token::Plus, self.cursor.span(1))),
            '*' => Some((Token::Star, self.cursor.span(1))),
            '/' => Some((Token::Slash, self.cursor.span(1))),
            '&' => Some((Token::And, self.cursor.span(1))),
            '#' => Some((Token::Hash, self.cursor.span(1))),
            '!' => Some((Token::Exclamation, self.cursor.span(1))),
            '~' => Some((Token::Tilde, self.cursor.span(1))),

            '.' => {
                if self.cursor.peek().is_some_and(|v| v == '.')
                    && self.cursor.peek_ahead(1).is_some_and(|v| v == '.')
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((Token::Ellipsis, span))
                } else if self.cursor.peek().is_some_and(|v| v == '.') {
                    let span = self.cursor.span(2);
                    self.cursor.skip(1);
                    Some((Token::Range, span))
                } else {
                    Some((Token::Dot, self.cursor.span(1)))
                }
            }

            _ => None,
        };

        if let Some(tkn) = sym {
            self.tokens.push(tkn);
            return Ok(());
        }

        let kw = match ch {
            'i' => {
                if let Some(next) = self.cursor.peek() {
                    let it = match next {
                        'f' => Some((Token::If, self.cursor.span(2))),
                        'd' => Some((Token::Id, self.cursor.span(2))),

                        'n' => {
                            if self.cursor.peek_ahead(1).is_some_and(|v| v.is_not_ident()) {
                                Some((Token::In, self.cursor.span(2)))
                            } else if self.cursor.peek_many(1, 2).is_some_and(|v| v == "it")
                                && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                            {
                                let span = self.cursor.span(4);
                                self.cursor.skip(2);
                                Some((Token::Init, span))
                            } else if self.cursor.peek_many(1, 4).is_some_and(|v| v == "line")
                                && self.cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                            {
                                let span = self.cursor.span(6);
                                self.cursor.skip(4);
                                Some((Token::Inline, span))
                            } else {
                                None
                            }
                        }

                        'm' => {
                            if self.cursor.peek_many(1, 4).is_some_and(|v| v == "port")
                                && self.cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                            {
                                let span = self.cursor.span(6);
                                self.cursor.skip(4);
                                Some((Token::Import, span))
                            } else {
                                None
                            }
                        }

                        _ => None,
                    };

                    if it.is_some() {
                        self.cursor.skip(1);
                    }

                    it
                } else {
                    None
                }
            }

            's' => {
                if self.cursor.peek_many(0, 2).is_some_and(|v| v == "ub")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((Token::Sub, span))
                } else if self.cursor.peek_many(0, 4).is_some_and(|v| v == "tore")
                    && self.cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(5);
                    self.cursor.skip(4);
                    Some((Token::Store, span))
                } else if self.cursor.peek_many(0, 7).is_some_and(|v| v == "elector")
                    && self.cursor.peek_ahead(7).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(8);
                    self.cursor.skip(7);
                    Some((Token::Selector, span))
                } else {
                    None
                }
            }

            'e' => {
                if self.cursor.peek_many(0, 5).is_some_and(|v| v == "xport")
                    && self.cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(6);
                    self.cursor.skip(5);
                    Some((Token::Export, span))
                } else if self.cursor.peek_many(0, 3).is_some_and(|v| v == "num")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((Token::Enum, span))
                } else if self.cursor.peek_many(0, 3).is_some_and(|v| v == "lse")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((Token::Else, span))
                } else {
                    None
                }
            }

            'f' => {
                if self.cursor.peek_many(0, 1).is_some_and(|v| v == "n")
                    && self.cursor.peek_ahead(1).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(2);
                    self.cursor.skip(1);
                    Some((Token::Fn, span))
                } else if self.cursor.peek_many(0, 2).is_some_and(|v| v == "or")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((Token::For, span))
                } else if self.cursor.peek_many(0, 5).is_some_and(|v| v == "acade")
                    && self.cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(6);
                    self.cursor.skip(5);
                    Some((Token::Facade, span))
                } else if self.cursor.peek_many(0, 4).is_some_and(|v| v == "alse")
                    && self.cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(5);
                    self.cursor.skip(4);
                    Some((Token::Bool(false), span))
                } else {
                    None
                }
            }

            'p' => {
                if self.cursor.peek_many(0, 2).is_some_and(|v| v == "ub")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((Token::Pub, span))
                } else if self.cursor.peek_many(0, 2).is_some_and(|v| v == "os")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((Token::Pos, span))
                } else if self.cursor.peek_many(0, 3).is_some_and(|v| v == "ath")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((Token::Path, span))
                } else {
                    None
                }
            }

            'c' => {
                if self.cursor.peek_many(0, 4).is_some_and(|v| v == "onst")
                    && self.cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(5);
                    self.cursor.skip(4);
                    Some((Token::Const, span))
                } else if self.cursor.peek_many(0, 7).is_some_and(|v| v == "ompiler")
                    && self.cursor.peek_ahead(7).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(8);
                    self.cursor.skip(7);
                    Some((Token::Compiler, span))
                } else if self.cursor.peek_many(0, 8).is_some_and(|v| v == "omponent")
                    && self.cursor.peek_ahead(8).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(9);
                    self.cursor.skip(8);
                    Some((Token::Component, span))
                } else if self.cursor.peek_ahead(1).is_some_and(|v| v.is_not_ident()) {
                    let span = self.cursor.span(1);
                    Some((Token::ComponentShort, span))
                } else {
                    None
                }
            }

            'l' => {
                if self.cursor.peek_many(0, 2).is_some_and(|v| v == "et")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((Token::Let, span))
                } else {
                    None
                }
            }

            'r' => {
                if self.cursor.peek_many(0, 5).is_some_and(|v| v == "eturn")
                    && self.cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(6);
                    self.cursor.skip(5);
                    Some((Token::Return, span))
                } else if self.cursor.peek_many(0, 2).is_some_and(|v| v == "ef")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((Token::Ref, span))
                } else {
                    None
                }
            }

            'o' => {
                if self.cursor.peek_many(0, 8).is_some_and(|v| v == "bjective")
                    && self.cursor.peek_ahead(8).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(9);
                    self.cursor.skip(8);
                    Some((Token::Objective, span))
                } else {
                    None
                }
            }

            'm' => {
                if self.cursor.peek_many(0, 5).is_some_and(|v| v == "odule")
                    && self.cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(6);
                    self.cursor.skip(5);
                    Some((Token::Module, span))
                } else {
                    None
                }
            }

            'n' => {
                if self.cursor.peek_many(0, 2).is_some_and(|v| v == "bt")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((Token::Nbt, span))
                } else {
                    None
                }
            }

            'g' => {
                if self.cursor.peek_many(0, 3).is_some_and(|v| v == "oto")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((Token::Goto, span))
                } else {
                    None
                }
            }

            't' => {
                if self.cursor.peek_many(0, 3).is_some_and(|v| v == "ick")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((Token::Tick, span))
                } else if self.cursor.peek_many(0, 3).is_some_and(|v| v == "rue")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((Token::Bool(true), span))
                } else {
                    None
                }
            }

            _ => None,
        };

        if let Some(tkn) = kw {
            self.tokens.push(tkn);

            return Ok(());
        }

        if ch == '"' {
            let mut s = Vec::new();
            let c2 = self.cursor.clone();

            while let Some(tkn) = self.cursor.next() {
                if tkn == '\\' && self.cursor.peek().is_some_and(|v| v == '"') {
                    self.cursor.skip(1);
                    s.push('"');
                    continue;
                }

                if tkn == '"' {
                    break;
                } else {
                    s.push(tkn);
                }
            }

            self.tokens
                .push((Token::String(s.iter().collect()), c2.span(s.len() + 2)));

            return Ok(());
        }

        if ch.is_ascii_digit() {
            let mut buf = Vec::new();
            let c2 = self.cursor.clone();

            buf.push(ch);

            while let Some(tkn) = self.cursor.peek() {
                if tkn.is_ascii_digit() || (tkn == '.' && !buf.contains(&'.')) {
                    buf.push(tkn);
                    self.cursor.skip(1);
                } else {
                    break;
                }
            }

            let span = c2.span(buf.len());

            if buf.contains(&'.') {
                if let Ok(it) = buf.iter().collect::<String>().parse() {
                    self.tokens.push((Token::Float(it), span));
                } else {
                    return Err(TokenizerError {
                        src: self.cursor.source(),
                        at: span,
                        err: format!(
                            "Could not parse a float: {}",
                            buf.iter().collect::<String>()
                        ),
                    }
                    .into());
                }
            } else {
                if let Ok(it) = buf.iter().collect::<String>().parse() {
                    self.tokens.push((Token::Int(it), span));
                } else {
                    return Err(TokenizerError {
                        src: self.cursor.source(),
                        at: span,
                        err: format!("Could not parse an int: {}", buf.iter().collect::<String>()),
                    }
                    .into());
                }
            }

            return Ok(());
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut ident = Vec::new();
            let c2 = self.cursor.clone();

            ident.push(ch);

            while let Some(tkn) = self.cursor.peek() {
                if tkn.is_ascii_alphanumeric() || tkn == '_' {
                    ident.push(tkn);
                    self.cursor.skip(1);
                } else {
                    break;
                }
            }

            self.tokens
                .push((Token::Ident(ident.iter().collect()), c2.span(ident.len())));

            return Ok(());
        }

        Err(TokenizerError {
            src: self.cursor.source(),
            at: self.cursor.span(1),
            err: format!("Unexpected character: {}", ch),
        }
        .into())
    }
}
