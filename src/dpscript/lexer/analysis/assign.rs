use super::Analyzer;
use crate::{AddSpan, Assign, LexerError, Node, Result, Spanned, Token, TokenCursor};

impl Analyzer<Assign> for Assign {
    fn analyze(
        mut item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> Result<Option<Assign>> {
        if !cursor
            .peek_until_if(
                |it| it.0 == Token::Equal,
                |it| match it.0 {
                    Token::Dot | Token::LeftBracket | Token::RightBracket | Token::Ident(_) => true,
                    _ => false,
                },
            )
            .is_some()
        {
            return Ok(None);
        }

        let mut span = item.1;
        let mut name_buf = Vec::new();
        let mut name_span = span;

        while let Some((tkn, sp)) = cursor.next() {
            if tkn == Token::Equal {
                break;
            }

            name_buf.push(tkn);
            name_span = name_span.add(sp);
        }

        let target = (
            name_buf
                .into_iter()
                .map(|it| format!("{}", it))
                .collect::<Vec<_>>()
                .join(""),
            name_span,
        );

        span = span.add(name_span);

        // let mut target = match name {
        //     Token::Ident(id) => (id, name_span),

        //     _ => {
        //         return Err(LexerError {
        //             src: cursor.source(),
        //             at: name_span,
        //             err: format!("Unexpected token while parsing an assignment: {}", name),
        //         }
        //         .into());
        //     }
        // };

        let mut buf = Vec::new();

        while let Some((tkn, span)) = cursor.next() {
            if tkn == Token::Semi {
                break;
            }

            buf.push((tkn, span));
        }

        span = span.add(buf.last().clone().unwrap().1);

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
                    target,
                    span,
                    value: Box::new(value.clone()),
                }))
            }

            None => Err(LexerError {
                src: cursor.source(),
                at: span,
                err: format!("Could not parse an assignment value!"),
            }
            .into()),
        }
    }
}
