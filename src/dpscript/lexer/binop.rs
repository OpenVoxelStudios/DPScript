use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            binop::{BinaryOpNode, BinaryOperation},
            node::Node,
        },
        lexer::{Result, err::LexerErr, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

// This code... is ass.
// Please fix.
// If you don't fix it, and instead make the problem worse, please increment this counter.
// Times the problem grew: 3

impl Lexer {
    pub fn read_binop(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to parse binary op...", self.nesting);

        let Some(last) = self.last.last() else {
            return Err(LexerErr::NoLastExpr { span: self.loc() });
        };

        let lhs = Box::new(last.to_owned());

        let (op_tkn, _) = self.start_parse_any(vec![
            Token::Plus,        // +
            Token::Minus,       // -
            Token::Star,        // *
            Token::Slash,       // /
            Token::Xor,         // ^
            Token::Modulo,      // %
            Token::Exclamation, // !=
            Token::Equal,       // ==
            Token::LeftAngle,   // <, <=
            Token::RightAngle,  // >, >=
        ])?;

        let op = match op_tkn {
            Token::Plus => BinaryOperation::Add,
            Token::Minus => BinaryOperation::Sub,
            Token::Star => BinaryOperation::Mul,
            Token::Slash => BinaryOperation::Div,
            Token::Xor => BinaryOperation::BitXor,
            Token::Modulo => BinaryOperation::Mod,

            Token::Exclamation => {
                self.start_parse(Token::Equal)?;
                BinaryOperation::CondNeq
            }

            Token::Equal => {
                self.start_parse(Token::Equal)?;
                BinaryOperation::CondEq
            }

            Token::LeftAngle => {
                if self.if_next_and_eat(Token::Equal) {
                    BinaryOperation::CondLe
                } else {
                    BinaryOperation::CondLt
                }
            }

            Token::RightAngle => {
                if self.if_next_and_eat(Token::Equal) {
                    BinaryOperation::CondGe
                } else {
                    BinaryOperation::CondGt
                }
            }

            _ => {
                unreachable!("This is a compiler bug, please report it! This should NEVER happen!")
            }
        };

        self.last.pop();
        self.nesting += 1;

        let rhs = Box::new(self.read_value()?);

        self.nesting -= 1;

        debug!("[{}] Successfully parsed a binary op!", self.nesting);

        self.pop_in_place()?;

        Ok(Node::BinaryOp(BinaryOpNode {
            span: lhs.span().add(rhs.span()),
            lhs,
            op,
            rhs,
        }))
    }

    pub fn read_binop_cond(&mut self) -> Result<Node> {
        self.push();

        debug!(
            "[{}] Attempting to parse binary op condition...",
            self.nesting
        );

        let Some(last) = self.last.last() else {
            return Err(LexerErr::NoLastExpr { span: self.loc() });
        };

        let lhs = Box::new(last.to_owned());

        let (op_tkn, _) = self.start_parse_any(vec![
            Token::Or,  // |, ||
            Token::And, // &, &&
        ])?;

        let op = match op_tkn {
            Token::Or => {
                if self.if_next_and_eat(Token::Or) {
                    BinaryOperation::CondOr
                } else {
                    BinaryOperation::BitOr
                }
            }

            Token::And => {
                if self.if_next_and_eat(Token::And) {
                    BinaryOperation::CondAnd
                } else {
                    BinaryOperation::BitAnd
                }
            }

            _ => {
                unreachable!("This is a compiler bug, please report it! This should NEVER happen!")
            }
        };

        self.last.pop();
        self.nesting += 1;

        let rhs = Box::new(self.read_value()?);

        self.nesting -= 1;

        debug!(
            "[{}] Successfully parsed a binary op condition!",
            self.nesting
        );

        self.pop_in_place()?;

        Ok(Node::BinaryOp(BinaryOpNode {
            span: lhs.span().add(rhs.span()),
            lhs,
            op,
            rhs,
        }))
    }

    pub fn read_binop_val(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to parse binary op value...", self.nesting);

        let Some(last) = self.last.last() else {
            return Err(LexerErr::NoLastExpr { span: self.loc() });
        };

        let mut lhs = Box::new(last.to_owned());

        let (op_tkn, _) = self.start_parse_any(vec![
            Token::Dot,         // lhs.rhs
            Token::LeftBracket, // lhs[rhs]
            Token::Range,       // lhs .. rhs
        ])?;

        let mut op = match op_tkn {
            Token::Dot => BinaryOperation::Field,
            Token::LeftBracket => BinaryOperation::ArrayIndex,
            Token::Range => BinaryOperation::Range,

            _ => {
                unreachable!("This is a compiler bug, please report it! This should NEVER happen!")
            }
        };

        let popped = self.last.pop();

        self.nesting += 1;

        let mut rhs = Box::new(self.read_value()?);

        self.nesting -= 1;

        if op == BinaryOperation::ArrayIndex {
            // re-push it and then pop it, otherwise the context won't be restored.
            if let Some(popped) = popped {
                self.last.push(popped);
            }

            self.start_parse(Token::RightBracket)?;
            self.last.pop();
        }

        debug!("[{}] Successfully parsed a binary op value!", self.nesting);

        // FIXUP! THE OUTPUT IF KEPT LIKE THIS IS BROKEN AS CRAP!
        // binop<O: a, binop<P: b, c>> -> binop<P: binop<O: a, b>, c>

        if let Node::BinaryOp(rhs_op) = *rhs {
            if rhs_op.op != BinaryOperation::ArrayIndex {
                // This is REALLY frickin hacky.
                // There HAS to be a better way.

                // binary<ArrayIndex, binary<Field, math_entity, binary<Field, transformation, translation>>, 0>
                // needs to become
                // binary<ArrayIndex, binary<Field, binary<Field, math_entity, transformation>, translation>, 0>

                // isolated:
                // binary<Field, math_entity, binary<Field, transformation, translation>>
                // needs to become
                // binary<Field, binary<Field, math_entity, transformation>, translation>

                // basically, binary operations NEED to be left-aligned

                lhs = Box::new(Node::BinaryOp(BinaryOpNode {
                    span: lhs.span().add(rhs_op.lhs.span()),
                    lhs: lhs,
                    op,
                    rhs: rhs_op.lhs,
                }));

                op = rhs_op.op;
                rhs = rhs_op.rhs;
            } else {
                rhs = Box::new(Node::BinaryOp(rhs_op));
            }
        }

        self.pop_in_place()?;

        Ok(Node::BinaryOp(BinaryOpNode {
            span: lhs.span().add(rhs.span()),
            lhs,
            op,
            rhs,
        }))
    }

    pub fn read_assign(&mut self) -> Result<Node> {
        self.push();

        debug!("[{}] Attempting to parse assignment op...", self.nesting);

        let lhs = Box::new(self.read_value()?);

        let (op_tkn, _) = self.start_parse_any(vec![
            Token::Plus,   // +=
            Token::Minus,  // -=
            Token::Star,   // *=
            Token::Slash,  // /=
            Token::Or,     // |=
            Token::Xor,    // ^=
            Token::Equal,  // =
            Token::Modulo, // %=
            Token::And,    // &=
        ])?;

        if op_tkn != Token::Equal {
            self.start_parse(Token::Equal)?;
        }

        self.nesting += 1;

        let rhs = Box::new(self.read_value()?);

        self.nesting -= 1;

        let op = match op_tkn {
            Token::Plus => BinaryOperation::AddAssign,
            Token::Minus => BinaryOperation::SubAssign,
            Token::Star => BinaryOperation::MulAssign,
            Token::Slash => BinaryOperation::DivAssign,
            Token::Or => BinaryOperation::BitOrAssign,
            Token::Xor => BinaryOperation::BitXorAssign,
            Token::Equal => BinaryOperation::Assign,
            Token::Modulo => BinaryOperation::ModAssign,
            Token::And => BinaryOperation::BitAndAssign,

            _ => {
                unreachable!("This is a compiler bug, please report it! This should NEVER happen!")
            }
        };

        debug!("[{}] Successfully parsed an assignment op!", self.nesting);

        self.pop_in_place()?;

        Ok(Node::BinaryOp(BinaryOpNode {
            span: lhs.span().add(rhs.span()),
            lhs,
            op,
            rhs,
        }))
    }
}
