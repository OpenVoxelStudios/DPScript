use crate::{
    AddSpan, LexerError, Node, Result, Spanned, Subroutine, Token, TokenCursor, check_token,
};

use super::Analyzer;

impl Analyzer<Subroutine> for Subroutine {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Subroutine>> {
        if item.0 != Token::Sub {
            return Ok(None);
        }

        let mut span = item.1;
        let (name, name_span) = cursor.next_or_die(item.1)?;

        let name = match name {
            Token::Ident(id) => (id, name_span),

            _ => {
                return Err(LexerError {
                    src: cursor.source(),
                    at: name_span,
                    err: format!("Unexpected token while parsing a subroutine: {}", name),
                }
                .into());
            }
        };

        check_token!(remove cursor == LeftBrace);

        let mut body = Vec::new();
        let mut buf = Vec::new();
        let mut opens = 0;

        while let Some((token, span)) = cursor.next() {
            if token == Token::LeftBrace {
                opens += 1;
            }

            if token == Token::RightBrace {
                if opens == 0 {
                    break;
                } else {
                    opens -= 1;
                }
            }

            buf.push((token, span));
        }

        if let Some(tkn) = buf.last() {
            span = span.add(tkn.1);
        }

        let mut buf_cursor =
            TokenCursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        while let Some(item) = buf_cursor.next() {
            Node::analyze(item, &mut buf_cursor, &mut body)?;
        }

        Ok(Some(Self {
            name,
            body,
            span,
            locals: None,
        }))
    }
}
