use crate::{
    AddSpan, LexerError, Literal, Node, Result, Spanned, Token, TokenCursor, check_token,
    take_until,
};

use super::Analyzer;

impl Analyzer<Literal> for Literal {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Literal>> {
        Ok(match item.0 {
            Token::Int(i) => Some(Literal::Int((i, item.1))),
            Token::Float(f) => Some(Literal::Float((f, item.1))),
            Token::Double(d) => Some(Literal::Double((d, item.1))),

            Token::Bool(b) => Some(Literal::Bool((b, item.1))),
            Token::String(s) => Some(Literal::String((s, item.1))),

            Token::Component => {
                let (_, s) = check_token!(remove cursor == Colon).unwrap();
                let (tkn, span) = cursor.next_or_die(s)?;

                Some(Literal::Component((
                    match tkn {
                        Token::String(s) => s,

                        _ => {
                            return Err(LexerError {
                                src: cursor.source(),
                                at: span,
                                err: format!("Expected string, got: {}", tkn),
                            }
                            .into());
                        }
                    },
                    span,
                )))
            }

            Token::Id => {
                let (_, s) = check_token!(remove cursor == Colon).unwrap();
                let (tkn, span) = cursor.next_or_die(s)?;

                Some(Literal::Identifier((
                    match tkn {
                        Token::String(s) => s,

                        _ => {
                            return Err(LexerError {
                                src: cursor.source(),
                                at: span,
                                err: format!("Expected string, got: {}", tkn),
                            }
                            .into());
                        }
                    },
                    span,
                )))
            }

            Token::Path => {
                let (_, s) = check_token!(remove cursor == Colon).unwrap();
                let (tkn, span) = cursor.next_or_die(s)?;

                Some(Literal::Path((
                    match tkn {
                        Token::String(s) => s,

                        _ => {
                            return Err(LexerError {
                                src: cursor.source(),
                                at: span,
                                err: format!("Expected string, got: {}", tkn),
                            }
                            .into());
                        }
                    },
                    span,
                )))
            }

            Token::Store => {
                let (_, s) = check_token!(remove cursor == Colon).unwrap();
                let (tkn, span) = cursor.next_or_die(s)?;

                Some(Literal::Store((
                    match tkn {
                        Token::String(s) => s,

                        _ => {
                            return Err(LexerError {
                                src: cursor.source(),
                                at: span,
                                err: format!("Expected string, got: {}", tkn),
                            }
                            .into());
                        }
                    },
                    span,
                )))
            }

            Token::Pos => {
                let (_, s) = check_token!(remove cursor == Colon).unwrap();
                let (_, s2) = check_token!(remove cursor == LeftBracket).unwrap();
                let mut span = s.add(s2);
                let mut x = Vec::new();
                let mut y = Vec::new();
                let mut z = Vec::new();

                take_until!(cursor, x, span => Comma);
                take_until!(cursor, y, span => Comma);

                let mut opens = 0;

                while let Some((tkn, sp)) = cursor.next() {
                    if tkn == Token::RightBracket {
                        if opens == 0 {
                            span = span.add(sp);
                            break;
                        } else {
                            opens -= 1;
                        }
                    }

                    if tkn == Token::LeftBracket {
                        opens += 1;
                    }

                    z.push((tkn, sp));
                }

                let mut x_out = Vec::new();
                let mut y_out = Vec::new();
                let mut z_out = Vec::new();

                let mut x_cursor = TokenCursor::new_from_src(
                    cursor.source().name(),
                    cursor.source().inner().clone(),
                    x,
                );

                let mut y_cursor = TokenCursor::new_from_src(
                    cursor.source().name(),
                    cursor.source().inner().clone(),
                    y,
                );

                let mut z_cursor = TokenCursor::new_from_src(
                    cursor.source().name(),
                    cursor.source().inner().clone(),
                    z,
                );

                while let Some(item) = x_cursor.next() {
                    Node::analyze(item, &mut x_cursor, &mut x_out)?;
                }

                while let Some(item) = y_cursor.next() {
                    Node::analyze(item, &mut y_cursor, &mut y_out)?;
                }

                while let Some(item) = z_cursor.next() {
                    Node::analyze(item, &mut z_cursor, &mut z_out)?;
                }

                Some(Literal::Pos3(x_out, y_out, z_out, span.add(s)))
            }

            Token::Selector => {
                let (_, s) = check_token!(remove cursor == Colon).unwrap();
                let (tkn, span) = cursor.next_or_die(s)?;

                Some(Literal::Selector((
                    match tkn {
                        Token::String(s) => s,

                        _ => {
                            return Err(LexerError {
                                src: cursor.source(),
                                at: span,
                                err: format!("Expected string, got: {}", tkn),
                            }
                            .into());
                        }
                    },
                    span,
                )))
            }

            Token::Nbt => {
                check_token!(remove cursor == Colon);
                check_token!(remove cursor == LeftBrace);

                let mut buf = Vec::new();
                let mut opens = 0;

                while let Some((tkn, span)) = cursor.next() {
                    if tkn == Token::RightBrace {
                        if opens == 0 {
                            break;
                        } else {
                            opens -= 1;
                        }
                    }

                    if tkn == Token::LeftBrace {
                        opens += 1;
                    }

                    buf.push((tkn, span));
                }

                let mut span = item.1;

                if let Some(tkn) = buf.last() {
                    span = span.add(tkn.1);
                }

                // let buf = buf
                //     .iter()
                //     .map(|v| format!("{}", v.0))
                //     .collect::<Vec<_>>()
                //     .join("");

                // let buf = format!("{{{}}}", buf);

                // Some(Literal::Nbt((json5::from_str(&buf)?, span)))

                // This is broken because json5 doesn't allow for variables, number suffixes notation, etc.
                // This'll require a lot of setup to handle objects and arrays and nested objects/arrays properly
                // as well as variables
                todo!("Dynamic NBT Literal Parsing")
            }

            Token::LeftBracket => {
                let mut buf = Vec::new();
                let mut cur = Vec::new();
                let mut opens = 0;
                let mut brace_opens = 0;
                let mut paren_opens = 0;

                while let Some((tkn, span)) = cursor.next() {
                    if tkn == Token::RightBracket {
                        if opens == 0 {
                            if !cur.is_empty() {
                                buf.push(cur);
                            }

                            break;
                        } else {
                            opens -= 1;
                        }
                    }

                    if tkn == Token::LeftBracket {
                        opens += 1;
                    }

                    if tkn == Token::RightBrace {
                        brace_opens -= 1;
                    }

                    if tkn == Token::LeftBrace {
                        brace_opens += 1;
                    }

                    if tkn == Token::RightParen {
                        paren_opens -= 1;
                    }

                    if tkn == Token::LeftParen {
                        paren_opens += 1;
                    }

                    if tkn == Token::Comma && opens == 0 && brace_opens == 0 && paren_opens == 0 {
                        buf.push(cur);
                        cur = Vec::new();
                        continue;
                    }

                    cur.push((tkn, span));
                }

                let mut span = item.1;
                let mut items = Vec::new();

                if let Some(tkns) = buf.last() {
                    if let Some(tkn) = tkns.last() {
                        span = span.add(tkn.1);
                    }
                }

                for buf in buf {
                    let mut buf_cursor = TokenCursor::new_from_src(
                        cursor.source().name(),
                        cursor.source().inner().clone(),
                        buf,
                    );

                    while let Some(item) = buf_cursor.next() {
                        Node::analyze(item, &mut buf_cursor, &mut items)?;
                    }
                }

                Some(Literal::Array((items, span)))
            }

            Token::Ident(name) => {
                if cursor.peek().is_some_and(|(v, _)| v == Token::Colon)
                    && cursor.peek_ahead(1).is_some_and(|(v, _)| v == Token::Colon)
                {
                    cursor.skip(2);

                    let tkn = cursor.next_or_die(item.1)?;

                    let val = match tkn.0 {
                        Token::Ident(id) => (id, tkn.1),
                        _ => {
                            return Err(LexerError {
                                src: cursor.source(),
                                at: tkn.1,
                                err: format!(
                                    "Unexpected token while parsing an enum value: {}",
                                    tkn.0
                                ),
                            }
                            .into());
                        }
                    };

                    Some(Literal::EnumValue((name, item.1), val))
                } else {
                    None
                }
            }

            _ => None,
        })
    }
}
