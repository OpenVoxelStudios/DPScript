use crate::{Type, TypeKind};
use miette::{SourceOffset, SourceSpan};
use std::fmt;

// TODO: Consolidate with `Type` and `TypeKind`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltInTypes {
    Identifier,
    NBTPath,
    Selector,
    Component,
    Objective,
    Store,
    NBT,
    Bool,
    Float,
    Double,
    Int,
    String,
    Any,
    Entity,
    Pos3,
    Enum(String),
    Array(Box<BuiltInTypes>),
    Void,
}

impl BuiltInTypes {
    pub fn create_fake_span(&self) -> SourceSpan {
        SourceSpan::new(
            SourceOffset::from_location(self.name(), 0, 0),
            self.name().len(),
        )
    }

    pub fn of(ty: TypeKind) -> Self {
        match ty {
            TypeKind::Int => Self::Int,
            TypeKind::Float => Self::Float,
            TypeKind::Double => Self::Double,
            TypeKind::Bool => Self::Bool,
            TypeKind::String => Self::String,
            TypeKind::NBT => Self::NBT,
            TypeKind::NBTPath => Self::NBTPath,
            TypeKind::Identifier => Self::Identifier,
            TypeKind::Selector => Self::Selector,
            TypeKind::Component => Self::Component,
            TypeKind::Store => Self::Store,
            TypeKind::Objective => Self::Objective,
            TypeKind::Any => Self::Any,
            TypeKind::Void => Self::Void,
            TypeKind::Entity => Self::Entity,
            TypeKind::Pos3 => Self::Pos3,
            TypeKind::Ident(it) => Self::Enum(it), // `Type::Ident`s should always be enums
            TypeKind::Array(it) => Self::Array(Box::new(it.kind.into())),
        }
    }

    pub fn as_type_kind(&self) -> TypeKind {
        match self {
            Self::Int => TypeKind::Int,
            Self::Float => TypeKind::Float,
            Self::Double => TypeKind::Double,
            Self::Bool => TypeKind::Bool,
            Self::String => TypeKind::String,
            Self::NBT => TypeKind::NBT,
            Self::NBTPath => TypeKind::NBTPath,
            Self::Identifier => TypeKind::Identifier,
            Self::Selector => TypeKind::Selector,
            Self::Component => TypeKind::Component,
            Self::Store => TypeKind::Store,
            Self::Objective => TypeKind::Objective,
            Self::Any => TypeKind::Any,
            Self::Void => TypeKind::Void,
            Self::Entity => TypeKind::Entity,
            Self::Pos3 => TypeKind::Pos3,
            Self::Enum(id) => TypeKind::Ident(id.clone()),
            Self::Array(it) => TypeKind::Array(Box::new(it.as_type(it.create_fake_span()))),
        }
    }

    pub fn as_type(&self, span: SourceSpan) -> Type {
        Type {
            kind: self.as_type_kind(),
            span,
        }
    }

    fn base_all() -> Vec<Self> {
        vec![
            Self::Identifier,
            Self::NBTPath,
            Self::Selector,
            Self::Component,
            Self::Objective,
            Self::Store,
            Self::Entity,
            Self::NBT,
            Self::Bool,
            Self::Float,
            Self::Int,
            Self::String,
            Self::Any,
            Self::Void,
            Self::Pos3,
        ]
    }

    pub fn all() -> Vec<Self> {
        let arrays = Self::base_all()
            .iter()
            .map(|v| BuiltInTypes::Array(Box::new(v.clone())))
            .collect::<Vec<_>>();

        let mut all = vec![
            Self::Identifier,
            Self::NBTPath,
            Self::Selector,
            Self::Component,
            Self::Objective,
            Self::Store,
            Self::Entity,
            Self::NBT,
            Self::Bool,
            Self::Float,
            Self::Int,
            Self::String,
            Self::Any,
            Self::Void,
            Self::Pos3,
        ];

        all.extend(arrays);

        all
    }

    pub fn names() -> Vec<String> {
        Self::all().iter().map(Self::name).collect()
    }

    pub fn name(&self) -> String {
        match self {
            Self::Identifier => "Identifier".into(),
            Self::NBTPath => "NBTPath".into(),
            Self::Selector => "Selector".into(),
            Self::Component => "Component".into(),
            Self::Objective => "Objective".into(),
            Self::Store => "Store".into(),
            Self::Entity => "Entity".into(),
            Self::NBT => "NBT".into(),
            Self::Bool => "bool".into(),
            Self::Float => "float".into(),
            Self::Double => "double".into(),
            Self::Int => "int".into(),
            Self::String => "str".into(),
            Self::Any => "Any".into(),
            Self::Enum(it) => it.into(),
            Self::Array(ty) => format!("{}[]", ty.name()),
            Self::Void => "Void".into(),
            Self::Pos3 => "Pos3".into(),
        }
    }

    pub fn methods(&self) -> Vec<(&'static str, Vec<BuiltInTypes>, Option<BuiltInTypes>)> {
        let mut base = match self {
            Self::Int => vec![
                ("add", vec![Self::Int], Some(Self::Int)),
                ("sub", vec![Self::Int], Some(Self::Int)),
                ("mul", vec![Self::Int], Some(Self::Int)),
                ("div", vec![Self::Int], Some(Self::Int)),
                ("eq", vec![Self::Int], Some(Self::Bool)),
                ("ltEq", vec![Self::Int], Some(Self::Bool)),
                ("gtEq", vec![Self::Int], Some(Self::Bool)),
                ("lt", vec![Self::Int], Some(Self::Bool)),
                ("gt", vec![Self::Int], Some(Self::Bool)),
                ("equals", vec![Self::Int], Some(Self::Bool)),
                ("lessThanEqual", vec![Self::Int], Some(Self::Bool)),
                ("greaterThanEqual", vec![Self::Int], Some(Self::Bool)),
                ("lessThan", vec![Self::Int], Some(Self::Bool)),
                ("greaterThan", vec![Self::Int], Some(Self::Bool)),
            ],

            Self::Float => vec![
                ("add", vec![Self::Float], Some(Self::Float)),
                ("sub", vec![Self::Float], Some(Self::Float)),
                ("mul", vec![Self::Float], Some(Self::Float)),
                ("div", vec![Self::Float], Some(Self::Float)),
                ("eq", vec![Self::Float], Some(Self::Bool)),
                ("ltEq", vec![Self::Float], Some(Self::Bool)),
                ("gtEq", vec![Self::Float], Some(Self::Bool)),
                ("lt", vec![Self::Float], Some(Self::Bool)),
                ("gt", vec![Self::Float], Some(Self::Bool)),
                ("equals", vec![Self::Float], Some(Self::Bool)),
                ("lessThanEqual", vec![Self::Float], Some(Self::Bool)),
                ("greaterThanEqual", vec![Self::Float], Some(Self::Bool)),
                ("lessThan", vec![Self::Float], Some(Self::Bool)),
                ("greaterThan", vec![Self::Float], Some(Self::Bool)),
            ],

            Self::Store => vec![
                ("set", vec![Self::NBTPath, Self::Any], None),
                ("get", vec![Self::NBTPath], Some(Self::Any)),
                ("append", vec![Self::NBTPath, Self::Any], None),
                ("remove", vec![Self::NBTPath], None),
            ],

            Self::Objective => vec![
                ("set", vec![Self::Entity, Self::Int], None),
                ("get", vec![Self::Entity], Some(Self::Int)),
                ("add", vec![Self::Entity, Self::Int], None),
                ("reset", vec![Self::Entity], None),
                (
                    "ltEq",
                    vec![Self::Entity, Self::Objective, Self::Entity],
                    Some(Self::Bool),
                ),
                (
                    "gtEq",
                    vec![Self::Entity, Self::Objective, Self::Entity],
                    Some(Self::Bool),
                ),
                (
                    "lt",
                    vec![Self::Entity, Self::Objective, Self::Entity],
                    Some(Self::Bool),
                ),
                (
                    "gt",
                    vec![Self::Entity, Self::Objective, Self::Entity],
                    Some(Self::Bool),
                ),
                (
                    "equals",
                    vec![Self::Entity, Self::Objective, Self::Entity],
                    Some(Self::Bool),
                ),
                (
                    "lessThanEqual",
                    vec![Self::Entity, Self::Objective, Self::Entity],
                    Some(Self::Bool),
                ),
                (
                    "greaterThanEqual",
                    vec![Self::Entity, Self::Objective, Self::Entity],
                    Some(Self::Bool),
                ),
                (
                    "lessThan",
                    vec![Self::Entity, Self::Objective, Self::Entity],
                    Some(Self::Bool),
                ),
                (
                    "greaterThan",
                    vec![Self::Entity, Self::Objective, Self::Entity],
                    Some(Self::Bool),
                ),
                (
                    "operation",
                    vec![
                        Self::Enum("ScoreboardOps".into()),
                        Self::Entity,
                        Self::Objective,
                        Self::Entity,
                    ],
                    None,
                ),
            ],

            Self::Array(it) => vec![
                ("add", vec![*it.clone()], None),
                ("get", vec![Self::Int], Some(*it.clone())),
                ("clear", vec![], None),
            ],

            Self::Component => vec![("and", vec![Self::NBT], Some(Self::Component))],
            Self::NBT => vec![("and", vec![Self::NBT], Some(Self::Component))],

            _ => vec![],
        };

        base.push(("cloneToTemp", vec![], Some(self.clone())));
        base
    }

    pub fn is_compatible(&self, other: &BuiltInTypes) -> bool {
        self.is_compatible_inner(other) || other.is_compatible_inner(self)
    }

    fn is_compatible_inner(&self, other: &BuiltInTypes) -> bool {
        match self {
            Self::Any => true,
            Self::NBT => *other == Self::NBT || Self::Component.is_compatible(other),

            Self::Array(it) => match other {
                BuiltInTypes::Array(other) => it.is_compatible(other),
                _ => false,
            },

            it => it == other,
        }
    }
}

impl From<TypeKind> for BuiltInTypes {
    fn from(value: TypeKind) -> Self {
        Self::of(value)
    }
}

impl From<Type> for BuiltInTypes {
    fn from(value: Type) -> Self {
        Self::of(value.kind)
    }
}

impl From<BuiltInTypes> for TypeKind {
    fn from(value: BuiltInTypes) -> Self {
        value.as_type_kind()
    }
}

impl From<BuiltInTypes> for Type {
    fn from(value: BuiltInTypes) -> Self {
        value.as_type(value.create_fake_span())
    }
}

impl fmt::Display for BuiltInTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
