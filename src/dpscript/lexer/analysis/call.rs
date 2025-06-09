use super::Analyzer;
use crate::{AddSpan, Call, LexerError, Node, Result, Spanned, Token, TokenCursor, check_token};

impl Analyzer<Call> for Call {
    fn analyze(
        mut item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> Result<Option<Call>> {
        let mut span = item.1.clone();

        let is_nested = match item.0 {
            Token::Ident(_) => true,
            _ => false,
        } && cursor.peek().is_some_and(|(v, _)| v == Token::Dot)
            && cursor.peek_ahead(1).is_some_and(|(v, _)| match v {
                Token::Ident(_) => true,
                _ => false,
            })
            && cursor
                .peek_ahead(2)
                .is_some_and(|(v, _)| v == Token::LeftParen);

        if !is_nested && !cursor.peek().is_some_and(|(v, _)| v == Token::LeftParen) {
            return Ok(None);
        }

        let mut parent = None;

        if is_nested {
            parent = match item.0 {
                Token::Ident(id) => Some((id, item.1)),
                _ => {
                    return Err(LexerError {
                        src: cursor.source(),
                        at: item.1,
                        err: format!("Unexpected token while parsing a nested call: {}", item.0),
                    }
                    .into());
                }
            };

            check_token!(remove cursor == Dot);
            item = cursor.next().unwrap();
        }

        let Some(function) = String::analyze(item, cursor, nodes)? else {
            return Ok(None);
        };

        check_token!(remove cursor == LeftParen);

        let mut args = Vec::new();

        if cursor.peek().is_some_and(|(v, _)| v != Token::RightParen) {
            let mut buf = Vec::new();
            let mut cur = Vec::new();
            let mut opens = 0;
            let mut brace_opens = 0;
            let mut bracket_opens = 0;

            while let Some((tkn, span)) = cursor.next() {
                if tkn == Token::RightParen {
                    if opens == 0 {
                        if !cur.is_empty() {
                            buf.push(cur);
                        }

                        break;
                    } else {
                        opens -= 1;
                    }
                }

                if tkn == Token::LeftParen {
                    opens += 1;
                }

                if tkn == Token::RightBracket {
                    bracket_opens -= 1;
                }

                if tkn == Token::LeftBracket {
                    bracket_opens += 1;
                }

                if tkn == Token::RightBrace {
                    brace_opens -= 1;
                }

                if tkn == Token::LeftBrace {
                    brace_opens += 1;
                }

                if tkn == Token::Comma && opens == 0 && brace_opens == 0 && bracket_opens == 0 {
                    buf.push(cur);
                    cur = Vec::new();
                    continue;
                }

                cur.push((tkn, span));
            }

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
                    Node::analyze(item, &mut buf_cursor, &mut args)?;
                }
            }
        } else {
            check_token!(remove cursor == RightParen);
        }

        Ok(Some(Self {
            function,
            args,
            span,
            parent,
        }))
    }
}
