use crate::{
    AddSpan, Conditional, LexerError, Node, Result, Spanned, Token, TokenCursor, check_token,
};

use super::Analyzer;

impl Analyzer<Conditional> for Conditional {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Conditional>> {
        if item.0 != Token::If {
            return Ok(None);
        }

        let mut span = item.1;
        let mut buf = Vec::new();
        let mut condition = Vec::new();

        while let Some((tkn, span)) = cursor.next() {
            if tkn == Token::LeftBrace {
                break;
            }

            buf.push((tkn, span));
        }

        let mut buf_cursor =
            TokenCursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        while let Some(item) = buf_cursor.next() {
            Node::analyze(item, &mut buf_cursor, &mut condition)?;
        }

        let Some(condition) = condition.last() else {
            return Err(LexerError {
                src: cursor.source(),
                at: span,
                err: "Condition does not return a value!".into(),
            }
            .into());
        };

        let condition = condition.clone();

        let mut buf = Vec::new();
        let mut body = Vec::new();
        let mut opens = 0;

        while let Some((tkn, span)) = cursor.next() {
            if tkn == Token::RightBrace {
                if opens == 0 {
                    break;
                } else {
                    opens += 1;
                }
            }

            if tkn == Token::LeftBrace {
                opens += 1;
            }

            buf.push((tkn, span));
        }

        if let Some((_, sp)) = buf.last() {
            span = span.add(sp.clone());
        }

        let mut buf_cursor =
            TokenCursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        while let Some(item) = buf_cursor.next() {
            Node::analyze(item, &mut buf_cursor, &mut body)?;
        }

        let mut else_body = Vec::new();

        if cursor.peek().is_some_and(|(v, _)| v == Token::Else) {
            cursor.skip(1);

            check_token!(remove cursor == LeftBrace);

            let mut buf = Vec::new();
            let mut opens = 0;

            while let Some((tkn, span)) = cursor.next() {
                if tkn == Token::RightBrace {
                    if opens == 0 {
                        break;
                    } else {
                        opens += 1;
                    }
                }

                if tkn == Token::LeftBrace {
                    opens += 1;
                }

                buf.push((tkn, span));
            }

            if let Some((_, sp)) = buf.last() {
                span = span.add(sp.clone());
            }

            let mut buf_cursor = TokenCursor::new_from_src(
                cursor.source().name(),
                cursor.source().inner().clone(),
                buf,
            );

            while let Some(item) = buf_cursor.next() {
                Node::analyze(item, &mut buf_cursor, &mut else_body)?;
            }
        }

        Ok(Some(Self {
            body,
            condition: Box::new(condition),
            else_body,
            span,
            if_locals: None,
            else_locals: None,
        }))
    }
}
