use crate::{
    AddSpan, LexerError, Node, Result, Spanned, Token, TokenCursor, Type, Variable, check_token,
};

use super::Analyzer;

impl Analyzer<Variable> for Variable {
    fn analyze(
        mut item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> Result<Option<Variable>> {
        let mut span = item.1;

        let is_pub = match item.0 {
            Token::Let | Token::Const => false,
            Token::Pub => true,
            _ => return Ok(None),
        };

        if is_pub {
            if !cursor
                .peek()
                .is_some_and(|(v, _)| v == Token::Let || v == Token::Const)
            {
                return Ok(None);
            }

            item = cursor.next().unwrap();
        }

        let is_const = match item.0 {
            Token::Let => false,
            Token::Const => true,
            _ => return Ok(None),
        };

        let (name, name_span) = cursor.next_or_die(item.1)?;

        let name = match name {
            Token::Ident(id) => (id, name_span),

            _ => {
                return Err(LexerError {
                    src: cursor.source(),
                    at: name_span,
                    err: format!("Unexpected token while parsing a variable: {}", name),
                }
                .into());
            }
        };

        let mut ty = None;

        if cursor.peek().is_some_and(|(v, _)| v == Token::Colon) {
            let (_, sp) = cursor.next().unwrap();
            let it = cursor.next_or_die(sp)?;

            ty = Type::analyze(it, cursor, nodes)?;
        }

        let it = check_token!(remove cursor == Equal).unwrap();

        let mut buf = Vec::new();

        while let Some((tkn, span)) = cursor.next() {
            if tkn == Token::Semi {
                break;
            }

            buf.push((tkn, span));
        }

        let mut nodes = Vec::new();

        let mut buf_cursor =
            TokenCursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        while let Some(item) = buf_cursor.next() {
            Node::analyze(item, &mut buf_cursor, &mut nodes)?;
        }

        // We use this method to allow `Operation` nodes to work
        let value = nodes.first();

        match value {
            Some(value) => {
                span = span.add(value.span());

                Ok(Some(Self {
                    is_pub,
                    is_const,
                    is_arg: false,
                    name,
                    ty,
                    span,
                    value: Some(Box::new(value.clone())),
                }))
            }

            None => Err(LexerError {
                src: cursor.source(),
                at: it.1,
                err: format!("Could not parse a variable value: {}", it.0),
            }
            .into()),
        }
    }
}
