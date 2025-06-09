use super::Analyzer;
use crate::{IRCommand, IRNode, IRToken, IRTokenCursor, Result, Spanned, check_ir_token};

impl Analyzer<IRCommand> for IRCommand {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRCommand>> {
        if item.0 == IRToken::Command {
            check_ir_token!(remove cursor == Colon);

            let mut buf = Vec::new();

            while let Some((tkn, span)) = cursor.next() {
                if tkn == IRToken::Semi {
                    break;
                }

                buf.push((tkn, span));
            }

            let mut items = Vec::new();
            let mut buf_cursor = IRTokenCursor::new_from_src(cursor.source(), buf);

            while let Some(item) = buf_cursor.next() {
                if item.0 == IRToken::Comma {
                    continue;
                }

                IRNode::analyze(item, &mut buf_cursor, &mut items)?;
            }

            Ok(Some(Self { cmd: items }))
        } else {
            Ok(None)
        }
    }
}
