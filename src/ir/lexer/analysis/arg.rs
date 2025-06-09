use super::Analyzer;
use crate::{
    IRArgumentOperation, IRGetArgument, IRNode, IRSetArgument, IRToken, IRTokenCursor, Result,
    Spanned, UnnamedLexerError, check_ir_token,
};

impl Analyzer<IRArgumentOperation> for IRArgumentOperation {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRArgumentOperation>> {
        if item.0 == IRToken::Argument {
            let it = cursor.next_or_die()?;

            Ok(match it.0 {
                IRToken::Get => {
                    check_ir_token!(remove cursor == Colon);

                    let it = cursor.next_or_die()?;

                    let index = match it.0 {
                        IRToken::Int(it) => it as usize,

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

                    let it = cursor.next_or_die()?;

                    let var = match it.0 {
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

                    Some(IRArgumentOperation::Get(IRGetArgument { index, var }))
                }

                IRToken::Set => {
                    check_ir_token!(remove cursor == Colon);

                    let it = cursor.next_or_die()?;

                    let index = match it.0 {
                        IRToken::Int(it) => it as usize,

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

                    let it = cursor.next_or_die()?;

                    let Some(val) = IRNode::analyze(it.clone(), cursor, &mut Vec::new())? else {
                        return Err(UnnamedLexerError {
                            src: cursor.source(),
                            at: it.1,
                            err: "Could not parse a node for a value!".into(),
                        }
                        .into());
                    };

                    check_ir_token!(remove cursor == Semi);

                    Some(IRArgumentOperation::Set(IRSetArgument {
                        index,
                        value: Box::new(val),
                    }))
                }

                IRToken::Clear => {
                    check_ir_token!(remove cursor == Semi);

                    Some(IRArgumentOperation::Clear)
                }

                _ => {
                    return Err(UnnamedLexerError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", it.0),
                    }
                    .into());
                }
            })
        } else {
            Ok(None)
        }
    }
}
