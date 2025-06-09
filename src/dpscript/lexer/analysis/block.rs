use crate::{AddSpan, Block, Node, Result, Spanned, Token, TokenCursor, check_token};

use super::Analyzer;

impl Analyzer<Block> for Block {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Block>> {
        let is_init = match item.0 {
            Token::Init => true,
            Token::Tick => false,
            _ => return Ok(None),
        };

        let is_tick = match item.0 {
            Token::Init => false,
            Token::Tick => true,
            _ => return Ok(None),
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
            is_init,
            is_tick,
            body,
            span,
            locals: None,
        }))
    }
}
