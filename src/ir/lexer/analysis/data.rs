use super::Analyzer;
use crate::{
    AddDataOperation, CopyDataOperation, IRConcat, IRDataOperation, IRNode, IRToken, IRTokenCursor,
    Result, Spanned, UnnamedLexerError, check_ir_token,
};

impl Analyzer<IRDataOperation> for IRDataOperation {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRDataOperation>> {
        if item.0 == IRToken::Data {
            let it = cursor.next_or_die()?;

            match it.0 {
                IRToken::Set => {
                    check_ir_token!(remove cursor == Colon);

                    let it = cursor.next_or_die()?;

                    let var = match it.0 {
                        IRToken::Ident(id) => id,

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
                        IRNode::analyze(item, &mut buf_cursor, &mut items)?;
                    }

                    let value = if items.len() == 1 {
                        items.first().unwrap().clone()
                    } else {
                        IRNode::Concat(IRConcat { items })
                    };

                    Ok(Some(Self::Set(AddDataOperation {
                        var,
                        value: Box::new(value),
                    })))
                }

                IRToken::Append => {
                    check_ir_token!(remove cursor == Colon);

                    let it = cursor.next_or_die()?;

                    let var = match it.0 {
                        IRToken::Ident(id) => id,

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
                        IRNode::analyze(item, &mut buf_cursor, &mut items)?;
                    }

                    let value = if items.len() == 1 {
                        items.first().unwrap().clone()
                    } else {
                        IRNode::Concat(IRConcat { items })
                    };

                    Ok(Some(Self::Append(AddDataOperation {
                        var,
                        value: Box::new(value),
                    })))
                }

                IRToken::Copy => {
                    check_ir_token!(remove cursor == Colon);

                    let it = cursor.next_or_die()?;

                    let source = match it.0 {
                        IRToken::Ident(id) => id,

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

                    let target = match it.0 {
                        IRToken::Ident(id) => id,

                        _ => {
                            return Err(UnnamedLexerError {
                                src: cursor.source(),
                                at: it.1,
                                err: format!("Unexpected token: {}", it.0),
                            }
                            .into());
                        }
                    };

                    Ok(Some(Self::Copy(CopyDataOperation { source, target })))
                }

                _ => {
                    return Err(UnnamedLexerError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", it.0),
                    }
                    .into());
                }
            }
        } else {
            Ok(None)
        }
    }
}
