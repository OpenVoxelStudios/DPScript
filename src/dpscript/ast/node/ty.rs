use std::fmt;

use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Type {
    pub span: SourceSpan,
    pub kind: TypeKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TypeKind {
    Int,
    Float,
    Double,
    Bool,
    String,
    NBT,
    NBTPath,
    Identifier,
    Selector,
    Component,
    Store,
    Objective,
    Any,
    Void,
    Entity,
    Pos3,
    Ident(String),
    Array(Box<Type>),
}

impl Type {
    pub fn is_compatible(&self, other: &Type) -> bool {
        self.kind.is_compatible(&other.kind)
    }

    pub fn array_element(&self) -> Option<Type> {
        self.kind.array_element()
    }
}

impl TypeKind {
    pub fn name(&self) -> String {
        match self.clone() {
            Self::Int => "int".into(),
            Self::Float => "float".into(),
            Self::Double => "double".into(),
            Self::Bool => "bool".into(),
            Self::String => "str".into(),
            Self::NBT => "NBT".into(),
            Self::NBTPath => "NBTPath".into(),
            Self::Identifier => "Identifier".into(),
            Self::Selector => "Selector".into(),
            Self::Component => "Component".into(),
            Self::Store => "Store".into(),
            Self::Objective => "Objective".into(),
            Self::Any => "Any".into(),
            Self::Void => "void".into(),
            Self::Entity => "Entity".into(),
            Self::Pos3 => "Pos3".into(),
            Self::Ident(id) => id,
            Self::Array(arr) => format!("{}[]", arr.kind.name()),
        }
    }

    pub fn is_compatible(&self, other: &TypeKind) -> bool {
        self.is_compatible_inner(other) || other.is_compatible_inner(self)
    }

    fn is_compatible_inner(&self, other: &TypeKind) -> bool {
        let compats = vec![
            Self::NBT,
            Self::Component,
            Self::Selector,
            Self::Entity,
            Self::Int,
            Self::Float,
        ];

        match self.clone() {
            Self::Any => true,

            Self::NBT
            | Self::Component
            | Self::Selector
            | Self::Entity
            | Self::Int
            | Self::Float => compats.contains(other),

            Self::Array(it) => match other {
                TypeKind::Array(other) => it.kind.is_compatible(&other.kind),
                _ => false,
            },

            it => it == other.clone(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Int | Self::Float => true,
            _ => false,
        }
    }

    pub fn array_element(&self) -> Option<Type> {
        match self.clone() {
            Self::Array(it) => Some(*it),
            _ => None,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
