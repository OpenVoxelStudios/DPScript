use super::Analyzer;
use crate::{
    IRLiteral, IRNode, IRToken, IRTokenCursor, Result, Spanned, UnnamedLexerError, check_ir_token,
};

impl Analyzer<IRLiteral> for IRLiteral {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRLiteral>> {
        Ok(match item.0 {
            IRToken::Literal(it) => Some(IRLiteral::String(it)),

            IRToken::Path => {
                if cursor
                    .peek()
                    .is_some_and(|(v, _)| v == IRToken::Exclamation)
                    && cursor
                        .peek_ahead(1)
                        .is_some_and(|(v, _)| v == IRToken::LeftParen)
                {
                    cursor.skip(2);

                    let tkn = cursor.next_or_die()?;

                    let IRToken::Ident(id) = tkn.0 else {
                        return Err(UnnamedLexerError {
                            src: cursor.source(),
                            at: tkn.1,
                            err: format!("Unexpected token: {}", tkn.0),
                        }
                        .into());
                    };

                    check_ir_token!(remove cursor == RightParen);

                    Some(IRLiteral::PathOf(id))
                } else {
                    None
                }
            }

            IRToken::Store => {
                if cursor
                    .peek()
                    .is_some_and(|(v, _)| v == IRToken::Exclamation)
                    && cursor
                        .peek_ahead(1)
                        .is_some_and(|(v, _)| v == IRToken::LeftParen)
                {
                    cursor.skip(2);

                    let tkn = cursor.next_or_die()?;

                    let IRToken::Ident(id) = tkn.0 else {
                        return Err(UnnamedLexerError {
                            src: cursor.source(),
                            at: tkn.1,
                            err: format!("Unexpected token: {}", tkn.0),
                        }
                        .into());
                    };

                    check_ir_token!(remove cursor == RightParen);

                    Some(IRLiteral::StoreOf(id))
                } else {
                    None
                }
            }

            _ => None,
        })
    }
}
