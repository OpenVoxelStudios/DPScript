use super::Analyzer;
use crate::{
    IRExecute, IRNode, IRToken, IRTokenCursor, Result, Spanned, UnnamedLexerError, check_ir_token,
};

impl Analyzer<IRExecute> for IRExecute {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRExecute>> {
        if item.0 == IRToken::Execute {
            check_ir_token!(remove cursor == Colon);

            let it = cursor.next_or_die()?;

            let selector = match it.0 {
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

            check_ir_token!(remove cursor == Comma);
            check_ir_token!(remove cursor == Dollar);

            let it = cursor.next_or_die()?;

            let block = match it.0 {
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

            Ok(Some(Self { block, selector }))
        } else {
            Ok(None)
        }
    }
}
