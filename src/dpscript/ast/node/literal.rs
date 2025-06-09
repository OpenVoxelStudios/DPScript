use super::Node;
use crate::{AddSpan, util::Spanned};
use miette::SourceSpan;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub enum Literal {
    Int(Spanned<i64>),
    Float(Spanned<f32>),
    Double(Spanned<f64>),
    Bool(Spanned<bool>),
    String(Spanned<String>),
    Component(Spanned<String>),
    Array(Spanned<Vec<Node>>),
    Identifier(Spanned<String>),
    Path(Spanned<String>),
    Store(Spanned<String>),
    Entity(Spanned<String>),
    Selector(Spanned<String>),
    Nbt(Spanned<Value>),
    Pos3(Vec<Node>, Vec<Node>, Vec<Node>, SourceSpan),

    /// An enum value. This is (Enum Name, Value Name).
    EnumValue(Spanned<String>, Spanned<String>),
}

impl Literal {
    pub fn span(&self) -> SourceSpan {
        match self.clone() {
            Self::Int((_, s)) => s,
            Self::Float((_, s)) => s,
            Self::Double((_, s)) => s,
            Self::Bool((_, s)) => s,
            Self::String((_, s)) => s,
            Self::Component((_, s)) => s,
            Self::Array((_, s)) => s,
            Self::Identifier((_, s)) => s,
            Self::Path((_, s)) => s,
            Self::Store((_, s)) => s,
            Self::Entity((_, s)) => s,
            Self::Selector((_, s)) => s,
            Self::EnumValue((_, s), (_, s2)) => s.add(s2),
            Self::Nbt((_, s)) => s,
            Self::Pos3(_, _, _, s) => s,
        }
    }
}
