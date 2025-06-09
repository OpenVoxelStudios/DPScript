use crate::{AddSpan, Enum, LexerError, Node, Result, Spanned, Token, TokenCursor, check_token};

use super::Analyzer;

impl Analyzer<Enum> for Enum {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Enum>> {
        let is_pub: bool = match item.0 {
            Token::Pub => true,
            Token::Enum => false,
            _ => return Ok(None),
        };

        if is_pub {
            if cursor.peek().is_some_and(|(v, _)| v == Token::Enum) {
                cursor.skip(1);
            } else {
                return Ok(None);
            }
        }

        let Some(name) = String::analyze(cursor.next_or_die(item.1)?, cursor, &mut Vec::new())?
        else {
            return Ok(None);
        };

        check_token!(remove cursor == LeftBrace);

        let mut buf = Vec::new();

        while let Some((tkn, span)) = cursor.next() {
            if tkn == Token::RightBrace {
                break;
            }

            if tkn == Token::Comma {
                continue;
            }

            buf.push((tkn, span));
        }

        let mut span = item.1;

        if let Some(tkn) = buf.last() {
            span = span.add(tkn.1);
        }

        let mut entries = Vec::new();
        let mut buf_cursor =
            TokenCursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        while let Some(item) = buf_cursor.next() {
            if let Some(it) = String::analyze(item.clone(), &mut buf_cursor, &mut Vec::new())? {
                entries.push(it);
            } else {
                return Err(LexerError {
                    src: cursor.source(),
                    at: item.1,
                    err: format!("Unexpected token while parsing enum: {}", item.0),
                }
                .into());
            }
        }

        Ok(Some(Self {
            is_pub,
            name,
            entries,
            span,
        }))
    }
}
