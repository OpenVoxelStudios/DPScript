use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{
            binop::{BinaryOpNode, BinaryOperation},
            node::Node,
        },
        lexer::{Result, parser::Lexer, util::LexerMethods},
        tokenizer::Token,
    },
    util::AddSpan,
};

impl Lexer {
    pub fn read_binop(&mut self) -> Result<Node> {
        self.push();

        debug!("Attempting to parse binary op...");

        let lhs = Box::new(self.read_value_nb()?);

        let (op_tkn, _) = self.start_parse_any(vec![
            Token::Plus,        // +
            Token::Minus,       // -
            Token::Star,        // *
            Token::Slash,       // /
            Token::Or,          // |, ||
            Token::Xor,         // ^
            Token::And,         // &, &&
            Token::Modulo,      // %
            Token::Exclamation, // !
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

        let rhs = Box::new(self.read_value()?);

        if op == BinaryOperation::ArrayIndex {
            self.expect(Token::RightBracket)?;
        }

        debug!("Successfully parsed a binary op!");

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

        debug!("Attempting to parse binary op value...");

        let lhs = Box::new(self.read_value_nbv()?);

        let (op_tkn, _) = self.start_parse_any(vec![
            Token::Dot,         // lhs.rhs
            Token::LeftBracket, // lhs[rhs]
        ])?;

        let op = match op_tkn {
            Token::Dot => BinaryOperation::Field,
            Token::LeftBracket => BinaryOperation::ArrayIndex,

            _ => {
                unreachable!("This is a compiler bug, please report it! This should NEVER happen!")
            }
        };

        let rhs = Box::new(self.read_value()?);

        if op == BinaryOperation::ArrayIndex {
            self.expect(Token::RightBracket)?;
        }

        debug!("Successfully parsed a binary op value!");

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

        debug!("Attempting to parse assignment op...");

        let lhs = Box::new(self.read_value_nb()?);

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

        let rhs = Box::new(self.read_value()?);

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

        debug!("Successfully parsed an assignment op!");

        self.pop_in_place()?;

        Ok(Node::BinaryOp(BinaryOpNode {
            span: lhs.span().add(rhs.span()),
            lhs,
            op,
            rhs,
        }))
    }
}
