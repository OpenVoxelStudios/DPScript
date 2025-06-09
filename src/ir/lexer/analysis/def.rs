use super::Analyzer;
use crate::{
    IRDefinition, IRNode, IRToken, IRTokenCursor, Result, Spanned, UnnamedLexerError,
    VariableAlias, check_ir_token,
};

impl Analyzer<IRDefinition> for IRDefinition {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRDefinition>> {
        if item.0 == IRToken::Define {
            let it = cursor.next_or_die()?;

            match it.0 {
                IRToken::VariableAlias => {
                    let it = cursor.next_or_die()?;

                    let name = match it.0 {
                        IRToken::Ident(it) => it,

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

                    let it = cursor.next_or_die()?;

                    let store = match it.0 {
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

                    check_ir_token!(remove cursor == At);

                    let it = cursor.next_or_die()?;

                    let path = match it.0 {
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

                    check_ir_token!(remove cursor == Semi);

                    Ok(Some(Self::VariableAlias(VariableAlias {
                        name,
                        path,
                        store,
                    })))
                }

                _ => Err(UnnamedLexerError {
                    src: cursor.source(),
                    at: it.1,
                    err: format!("Unexpected token: {}", it.0),
                }
                .into()),
            }
        } else {
            Ok(None)
        }
    }
}
