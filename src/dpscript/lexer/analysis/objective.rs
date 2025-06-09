use crate::{
    AddSpan, LexerError, Node, Objective, Result, Spanned, Token, TokenCursor, check_token,
};

use super::Analyzer;

impl Analyzer<Objective> for Objective {
    fn analyze(
        mut item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Objective>> {
        let mut span = item.1;

        let is_pub = match item.0 {
            Token::Objective => false,
            Token::Pub => true,
            _ => return Ok(None),
        };

        if is_pub {
            if !cursor.peek().is_some_and(|(v, _)| v == Token::Objective) {
                return Ok(None);
            }

            item = cursor.next().unwrap();
        }

        let (name, name_span) = cursor.next_or_die(item.1)?;

        let name = match name {
            Token::Ident(id) => (id, name_span),

            _ => {
                return Err(LexerError {
                    src: cursor.source(),
                    at: name_span,
                    err: format!("Unexpected token while parsing an objective: {}", name),
                }
                .into());
            }
        };

        let mut criteria = ("dummy".into(), name_span);

        if cursor.peek().is_some_and(|(v, _)| v == Token::Colon) {
            let (_, sp) = cursor.next().unwrap();
            let it = cursor.next_or_die(sp)?;

            if let Token::Ident(id) = it.0 {
                criteria = (id, it.1);
            }
        }

        let (_, sp) = check_token!(remove cursor == Equal).unwrap();
        let (tkn, sp) = cursor.next_or_die(sp)?;

        let id = match tkn {
            Token::String(s) => (s, sp),
            _ => {
                return Err(LexerError {
                    src: cursor.source(),
                    at: sp,
                    err: format!("Unexpected token while parsing an objective: {}", tkn),
                }
                .into());
            }
        };

        span = span.add(sp);

        Ok(Some(Self {
            criteria,
            id,
            is_pub,
            name,
            span,
        }))
    }
}
