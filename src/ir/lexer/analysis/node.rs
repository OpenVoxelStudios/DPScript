use super::Analyzer;
use crate::{
    IRArgumentOperation, IRBlock, IRCall, IRCommand, IRConcat, IRCondition, IRDataOperation,
    IRDefinition, IRExecute, IRFunction, IRLiteral, IRNode, IRTag, IRToken, IRTokenCursor, Result,
    Spanned, UnnamedLexerError, add_ir_return, check_ir_token,
};

impl Analyzer<IRNode> for IRNode {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRNode>> {
        if item.0 == IRToken::Semi {
            return Ok(None);
        }

        match IRDefinition::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Definition(v)),
            None => {}
        };

        match IRFunction::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Function(v)),
            None => {}
        };

        match IRConcat::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Concat(v)),
            None => {}
        };

        match IRArgumentOperation::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Argument(v)),
            None => {}
        };

        match IRTag::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Tag(v)),
            None => {}
        };

        match IRBlock::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Block(v)),
            None => {}
        };

        match IRCommand::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Command(v)),
            None => {}
        };

        match IRExecute::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Execute(v)),
            None => {}
        };

        match IRCondition::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Condition(v)),
            None => {}
        };

        match IRCall::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Call(v)),
            None => {}
        };

        match IRDataOperation::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += DataOperation(v)),
            None => {}
        };

        match IRLiteral::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Literal(v)),
            None => {}
        };

        if item.0 == IRToken::Goto {
            check_ir_token!(remove cursor == Colon);
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

            check_ir_token!(remove cursor == Semi);
            add_ir_return!(nodes += Goto(block));
        }

        if let IRToken::Ident(id) = item.0 {
            Ok(Some(IRNode::Reference(id)))
        } else {
            Err(UnnamedLexerError {
                src: cursor.source(),
                at: item.1,
                err: format!("Unexpected token while parsing an IRNode: {}", item.0),
            }
            .into())
        }
    }
}
