use super::Analyzer;
use crate::{
    IRCondition, IRNode, IRToken, IRTokenCursor, Result, Spanned, UnnamedLexerError, check_ir_token,
};

impl Analyzer<IRCondition> for IRCondition {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRCondition>> {
        if item.0 == IRToken::Condition {
            check_ir_token!(remove cursor == Colon);

            let it = cursor.next_or_die()?;

            let score = match it.0 {
                IRToken::Literal(it) => it,

                _ => {
                    return Err(UnnamedLexerError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", it.0),
                    }
                    .into());
                }
            };

            check_ir_token!(remove cursor == Semi);
            check_ir_token!(remove cursor == If);
            check_ir_token!(remove cursor == Colon);
            check_ir_token!(remove cursor == Dollar);

            let it = cursor.next_or_die()?;

            let if_block = match it.0 {
                IRToken::Ident(it) => it,

                _ => {
                    return Err(UnnamedLexerError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", it.0),
                    }
                    .into());
                }
            };

            check_ir_token!(remove cursor == Semi);
            check_ir_token!(remove cursor == Else);
            check_ir_token!(remove cursor == Colon);
            check_ir_token!(remove cursor == Dollar);

            let it = cursor.next_or_die()?;

            let else_block = match it.0 {
                IRToken::Ident(it) => it,

                _ => {
                    return Err(UnnamedLexerError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", it.0),
                    }
                    .into());
                }
            };

            check_ir_token!(remove cursor == Semi);
            check_ir_token!(remove cursor == Join);
            check_ir_token!(remove cursor == Colon);
            check_ir_token!(remove cursor == Dollar);

            let it = cursor.next_or_die()?;

            let join_block = match it.0 {
                IRToken::Ident(it) => it,

                _ => {
                    return Err(UnnamedLexerError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", it.0),
                    }
                    .into());
                }
            };

            check_ir_token!(remove cursor == Semi);

            Ok(Some(Self {
                score,
                else_block,
                if_block,
                join_block,
            }))
        } else {
            Ok(None)
        }
    }
}
