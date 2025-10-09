//! Binary operations

use std::fmt;

use dpscript_macros::HasSpan;
use miette::SourceSpan;

use crate::{
    dpscript::{
        ast::{ast::Scope, node::Node},
        data::NodeInfo,
        tokenizer::Token,
        ty::{BuiltInType, TypeRef},
    },
    util::Spanned,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, HasSpan)]
pub struct BinaryOpNode {
    pub span: SourceSpan,
    pub op: BinaryOperation,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BinaryOperation {
    /// lhs + rhs
    Add,

    /// lhs - rhs
    Sub,

    /// lhs * rhs
    Mul,

    /// lhs / rhs
    Div,

    /// lhs % rhs
    Mod,

    /// lhs & rhs
    BitAnd,

    /// lhs | rhs
    BitOr,

    /// lhs ^ rhs
    BitXor,

    /// lhs && rhs
    CondAnd,

    /// lhs || rhs
    CondOr,

    /// lhs == rhs
    CondEq,

    /// lhs != rhs
    CondNeq,

    /// lhs > rhs
    CondGt,

    /// lhs >= rhs
    CondGe,

    /// lhs < rhs
    CondLt,

    /// lhs <= rhs
    CondLe,

    /// lhs = rhs
    Assign,

    /// lhs += rhs
    AddAssign,

    /// lhs -= rhs
    SubAssign,

    /// lhs *= rhs
    MulAssign,

    /// lhs /= rhs
    DivAssign,

    /// lhs %= rhs
    ModAssign,

    /// lhs &= rhs
    BitAndAssign,

    /// lhs |= rhs
    BitOrAssign,

    /// lhs ^= rhs
    BitXorAssign,

    /// lhs .. rhs
    Range,

    /// lhs.rhs
    Field,

    /// lhs[rhs]
    ArrayIndex,
}

impl fmt::Display for BinaryOpNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(feature = "print-clarity"))]
        match self.op {
            BinaryOperation::Add => write!(f, "{} + {}", self.lhs, self.rhs),
            BinaryOperation::Sub => write!(f, "{} - {}", self.lhs, self.rhs),
            BinaryOperation::Mul => write!(f, "{} * {}", self.lhs, self.rhs),
            BinaryOperation::Div => write!(f, "{} / {}", self.lhs, self.rhs),
            BinaryOperation::Mod => write!(f, "{} % {}", self.lhs, self.rhs),
            BinaryOperation::BitAnd => write!(f, "{} & {}", self.lhs, self.rhs),
            BinaryOperation::BitOr => write!(f, "{} | {}", self.lhs, self.rhs),
            BinaryOperation::BitXor => write!(f, "{} ^ {}", self.lhs, self.rhs),
            BinaryOperation::CondAnd => write!(f, "{} && {}", self.lhs, self.rhs),
            BinaryOperation::CondOr => write!(f, "{} || {}", self.lhs, self.rhs),
            BinaryOperation::CondEq => write!(f, "{} == {}", self.lhs, self.rhs),
            BinaryOperation::CondNeq => write!(f, "{} != {}", self.lhs, self.rhs),
            BinaryOperation::CondGt => write!(f, "{} > {}", self.lhs, self.rhs),
            BinaryOperation::CondGe => write!(f, "{} >= {}", self.lhs, self.rhs),
            BinaryOperation::CondLt => write!(f, "{} < {}", self.lhs, self.rhs),
            BinaryOperation::CondLe => write!(f, "{} <= {}", self.lhs, self.rhs),
            BinaryOperation::Assign => write!(f, "{} = {};", self.lhs, self.rhs),
            BinaryOperation::AddAssign => write!(f, "{} += {};", self.lhs, self.rhs),
            BinaryOperation::SubAssign => write!(f, "{} -= {};", self.lhs, self.rhs),
            BinaryOperation::MulAssign => write!(f, "{} *= {};", self.lhs, self.rhs),
            BinaryOperation::DivAssign => write!(f, "{} /= {};", self.lhs, self.rhs),
            BinaryOperation::ModAssign => write!(f, "{} %= {};", self.lhs, self.rhs),
            BinaryOperation::BitAndAssign => write!(f, "{} &= {};", self.lhs, self.rhs),
            BinaryOperation::BitOrAssign => write!(f, "{} |= {};", self.lhs, self.rhs),
            BinaryOperation::BitXorAssign => write!(f, "{} ^= {};", self.lhs, self.rhs),
            BinaryOperation::Range => write!(f, "{} .. {}", self.lhs, self.rhs),
            BinaryOperation::Field => write!(f, "{}.{}", self.lhs, self.rhs),
            BinaryOperation::ArrayIndex => write!(f, "{}[{}]", self.lhs, self.rhs),
        }

        #[cfg(feature = "print-clarity")]
        match self.op {
            BinaryOperation::Add => write!(f, "binary<Add, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Sub => write!(f, "binary<Sub, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Mul => write!(f, "binary<Mul, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Div => write!(f, "binary<Div, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Mod => write!(f, "binary<Mod, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::BitAnd => write!(f, "binary<BitAnd, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::BitOr => write!(f, "binary<BitOr, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::BitXor => write!(f, "binary<BitXor, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondAnd => write!(f, "binary<CondAnd, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondOr => write!(f, "binary<CondOr, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondEq => write!(f, "binary<CondEq, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondNeq => write!(f, "binary<CondNeq, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondGt => write!(f, "binary<CondGt, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondGe => write!(f, "binary<CondGe, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondLt => write!(f, "binary<CondLt, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::CondLe => write!(f, "binary<CondLe, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Assign => write!(f, "binary<Assign, {}, {}>;", self.lhs, self.rhs),
            BinaryOperation::AddAssign => {
                write!(f, "binary<AddAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::SubAssign => {
                write!(f, "binary<SubAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::MulAssign => {
                write!(f, "binary<MulAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::DivAssign => {
                write!(f, "binary<DivAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::ModAssign => {
                write!(f, "binary<ModAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::BitAndAssign => {
                write!(f, "binary<BitAndAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::BitOrAssign => {
                write!(f, "binary<BitOrAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::BitXorAssign => {
                write!(f, "binary<BitXorAssign, {}, {}>;", self.lhs, self.rhs)
            }
            BinaryOperation::Range => write!(f, "binary<Range, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::Field => write!(f, "binary<Field, {}, {}>", self.lhs, self.rhs),
            BinaryOperation::ArrayIndex => {
                write!(f, "binary<ArrayIndex, {}, {}>", self.lhs, self.rhs)
            }
        }
    }
}

impl Token {
    pub fn is_binop_val(&self, next: Option<&Spanned<Token>>) -> bool {
        match self {
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Modulo
            | Self::And
            | Self::Or
            | Self::Xor => match next {
                Some((Self::Equal, _)) => false,
                _ => true,
            },

            Self::LeftAngle | Self::RightAngle | Self::Range | Self::Dot | Self::LeftBracket => {
                true
            }

            Self::Equal | Self::Exclamation => match next {
                Some((Self::Equal, _)) => true,
                _ => false,
            },

            _ => false,
        }
    }
}

impl BinaryOperation {
    /// Get the assignment form of this operation, otherwise panic.
    pub fn assign(&self) -> Self {
        match self {
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::BitAndAssign
            | Self::BitOrAssign
            | Self::BitXorAssign => *self,

            Self::Add => Self::AddAssign,
            Self::Sub => Self::SubAssign,
            Self::Mul => Self::MulAssign,
            Self::Div => Self::DivAssign,
            Self::Mod => Self::ModAssign,
            Self::BitAnd => Self::BitAndAssign,
            Self::BitOr => Self::BitOrAssign,
            Self::BitXor => Self::BitXorAssign,

            other => panic!(
                "Binary operation cannot be used as an assignment operator: {other:?}\nThis is a compiler bug! Please report this!"
            ),
        }
    }

    pub fn is_assign(&self) -> bool {
        match self {
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::BitAndAssign
            | Self::BitOrAssign
            | Self::BitXorAssign => true,

            _ => false,
        }
    }

    /// Determine if the operation itself is inherently constant.
    pub fn is_const(&self) -> bool {
        match self {
            Self::Assign
            | Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::BitAndAssign
            | Self::BitOrAssign
            | Self::BitXorAssign
            | Self::Range
            | Self::Field
            | Self::ArrayIndex => false,

            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::Mod
            | Self::BitAnd
            | Self::BitOr
            | Self::BitXor
            | Self::CondEq
            | Self::CondNeq
            | Self::CondGt
            | Self::CondGe
            | Self::CondLt
            | Self::CondLe
            | Self::CondAnd
            | Self::CondOr => true,
        }
    }
}

impl NodeInfo for BinaryOpNode {
    fn is_const(&self, scope: &Scope) -> bool {
        self.lhs.is_const(scope) && self.rhs.is_const(scope) && self.op.is_const()
    }

    fn returns(&self, scope: &Scope) -> Option<TypeRef> {
        match self.op {
            BinaryOperation::Add
            | BinaryOperation::Sub
            | BinaryOperation::Mul
            | BinaryOperation::Div
            | BinaryOperation::Mod
            | BinaryOperation::BitAnd
            | BinaryOperation::BitOr
            | BinaryOperation::BitXor => self.lhs.returns(scope),

            BinaryOperation::CondAnd
            | BinaryOperation::CondOr
            | BinaryOperation::CondEq
            | BinaryOperation::CondNeq
            | BinaryOperation::CondGt
            | BinaryOperation::CondGe
            | BinaryOperation::CondLt
            | BinaryOperation::CondLe => Some(TypeRef::BuiltIn(BuiltInType::Boolean)),

            BinaryOperation::Assign
            | BinaryOperation::AddAssign
            | BinaryOperation::SubAssign
            | BinaryOperation::MulAssign
            | BinaryOperation::DivAssign
            | BinaryOperation::ModAssign
            | BinaryOperation::BitAndAssign
            | BinaryOperation::BitOrAssign
            | BinaryOperation::BitXorAssign => None,

            BinaryOperation::Range => self
                .lhs
                .returns(scope)
                .map(|it| TypeRef::Array(Box::new(it))),

            BinaryOperation::Field => match &*self.rhs {
                Node::Ident(id) => scope
                    .module
                    .fields
                    .get(&self.lhs.returns(scope)?)?
                    .get(&id.ident)
                    .cloned(),
                _ => None,
            },

            BinaryOperation::ArrayIndex => match self.rhs.returns(scope)? {
                TypeRef::Array(ty) | TypeRef::SizedArray(ty, _) => Some(*ty),

                _ => None,
            },
        }
    }
}
