use super::Analyzer;
use crate::{
    IRNode, IRTag, IRToken, IRTokenCursor, Result, Spanned, UnnamedLexerError, check_ir_token,
};

impl Analyzer<IRTag> for IRTag {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRTag>> {
        if item.0 == IRToken::Tag {
            let it = cursor.next_or_die()?;

            let name = match it.0 {
                IRToken::Literal(it) => it,

                tkn => {
                    return Err(UnnamedLexerError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", tkn),
                    }
                    .into());
                }
            };

            check_ir_token!(remove cursor == Colon);
            check_ir_token!(remove cursor == LeftBracket);

            let mut buf = Vec::new();

            while let Some((tkn, span)) = cursor.next() {
                if tkn == IRToken::RightBracket {
                    break;
                }

                buf.push((tkn, span));
            }

            check_ir_token!(remove cursor == Semi);

            let mut entries = Vec::new();
            let mut buf_cursor = IRTokenCursor::new_from_src(cursor.source(), buf);

            while let Some(item) = buf_cursor.next() {
                if let IRToken::Literal(it) = item.0 {
                    entries.push(it);
                    check_ir_token!(remove buf_cursor == Semi);
                } else {
                    return Err(UnnamedLexerError {
                        src: cursor.source(),
                        at: item.1,
                        err: format!("Unexpected token: {}", item.0),
                    }
                    .into());
                }
            }

            Ok(Some(Self { name, entries }))
        } else {
            Ok(None)
        }
    }
}
