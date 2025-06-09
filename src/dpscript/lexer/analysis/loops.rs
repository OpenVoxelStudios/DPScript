use super::Analyzer;

use crate::{AddSpan, LexerError, Loop, Node, Result, Spanned, Token, TokenCursor, check_token};

impl Analyzer<Loop> for Loop {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Loop>> {
        if item.0 != Token::For {
            return Ok(None);
        }

        let (var_name, var_name_span) = cursor.next_or_die(item.1)?;

        let var_name = match var_name {
            Token::Ident(id) => (id, var_name_span),

            _ => {
                return Err(LexerError {
                    src: cursor.source(),
                    at: var_name_span,
                    err: format!("Unexpected token while parsing a loop: {}", var_name),
                }
                .into());
            }
        };

        check_token!(remove cursor == In);

        let (array, array_span) = cursor.next_or_die(var_name_span)?;

        let array = match array {
            Token::Ident(id) => (id, array_span),

            _ => {
                return Err(LexerError {
                    src: cursor.source(),
                    at: array_span,
                    err: format!("Unexpected token while parsing a loop: {}", array),
                }
                .into());
            }
        };

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

        let mut body = Vec::new();
        let mut buf_cursor =
            TokenCursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        while let Some(item) = buf_cursor.next() {
            Node::analyze(item, &mut buf_cursor, &mut body)?;
        }

        Ok(Some(Self {
            array,
            var_name,
            body,
            span,
            locals: None,
        }))
    }
}
