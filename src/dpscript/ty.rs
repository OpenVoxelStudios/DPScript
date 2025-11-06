use std::fmt;

use strum::{Display, EnumString};

use crate::util::Spanned;

#[derive(Debug, Clone, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TypeRef {
    /// A local (user-defined) type.
    Local(String),

    /// A type that is built-in to the language.
    BuiltIn(BuiltInType),

    /// A type for an array with a single type.
    Array(Box<TypeRef>),

    /// A type for an NBT array with a set length.
    SizedArray(Box<TypeRef>, Spanned<usize>),

    /// A type for NBT data with a specific schema.
    /// If this should instead be unvalidated, use [`BuiltInType::NBT`].
    TypedNBT(NBTSchema),
}

impl Eq for TypeRef {}

impl PartialEq for TypeRef {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Local(a) => match other {
                Self::Local(b) => a == b,
                Self::BuiltIn(BuiltInType::Any) => true,
                _ => false,
            },

            Self::BuiltIn(a) => {
                if *a == BuiltInType::Any {
                    true
                } else {
                    match other {
                        Self::BuiltIn(b) => {
                            a == b
                                || (a.is_numeric() && b.is_numeric())
                                || *b == BuiltInType::Any
                                || (*b == BuiltInType::Identifier && *a == BuiltInType::String)
                                || (*b == BuiltInType::Selector && *a == BuiltInType::String)
                        }

                        Self::TypedNBT(_) => matches!(a, BuiltInType::NBT),
                        _ => false,
                    }
                }
            }

            Self::Array(a) => match other {
                Self::Array(b) => a == b,
                Self::SizedArray(b, _) => a == b,
                Self::BuiltIn(BuiltInType::Any) => true,
                Self::BuiltIn(BuiltInType::NBT) => a.is_nbt(),
                _ => false,
            },

            Self::SizedArray(a, _) => match other {
                Self::SizedArray(b, _) => a == b,
                Self::BuiltIn(BuiltInType::Any) => true,
                Self::BuiltIn(BuiltInType::NBT) => a.is_nbt(),
                _ => false,
            },

            Self::TypedNBT(a) => match other {
                Self::TypedNBT(b) => a == b,
                // We have to, it has to be reflexive because of Eq for a BTreeMap.
                // TODO: Can we still do schema validation as long as it's on both sides?
                Self::BuiltIn(BuiltInType::NBT | BuiltInType::Any) => true,
                _ => false,
            },
        }
    }
}

// For strum variants - primitives should be lowercase, others shuold be PascalCase.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    EnumString,
    Display,
)]
pub enum BuiltInType {
    /// An integer type.
    #[strum(serialize = "int")]
    Int,

    /// A float type.
    #[strum(serialize = "float")]
    Float,

    /// A double type.
    #[strum(serialize = "double")]
    Double,

    /// A true/false boolean type.
    #[strum(serialize = "bool")]
    Boolean,

    /// A type for any arbitrary string.
    #[strum(serialize = "str")]
    String,

    /// A time type.
    /// Examples:
    ///  - 5s
    ///  - 12m38s
    ///  - 14d
    #[strum(serialize = "Time")]
    Time,

    /// A type for an identifier (or ResourceLocation).
    /// Should be a string in the form "namespace:path".
    #[strum(serialize = "Ident")]
    Identifier,

    /// A type for arbitrary NBT (named binary tag) data.
    #[strum(serialize = "NBT")]
    NBT,

    /// An entity selector.
    #[strum(serialize = "Selector")]
    Selector,

    /// A 3D world position.
    #[strum(serialize = "Pos3")]
    Pos,

    /// The 'Any' type. Allows any value.
    #[strum(serialize = "Any")]
    Any,

    /// The type of an objective. This is only used at compile time for type checking.
    #[strum(serialize = "Objective")]
    Objective,

    /// The void type. Represents nothing.
    #[strum(serialize = "void")]
    Void,

    /// An entity transform (rotation, translation, etc.)
    #[strum(serialize = "Transform")]
    Transform,
}

impl fmt::Display for TypeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeRef::Local(it) => write!(f, "Local<{it}>"),
            TypeRef::BuiltIn(it) => write!(f, "BuiltIn<{it}>"),
            TypeRef::Array(it) => write!(f, "SingleArray<{it}>"),
            TypeRef::SizedArray(it, size) => write!(f, "SizedArray<[{it}; {}]>", size.0),
            TypeRef::TypedNBT(_) => write!(f, "TypedNBT"),
        }
    }
}

impl TypeRef {
    pub fn is_numeric(&self) -> bool {
        match self {
            TypeRef::BuiltIn(it) => it.is_numeric(),

            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            TypeRef::BuiltIn(
                BuiltInType::Boolean | BuiltInType::Any | BuiltInType::Selector | BuiltInType::Int,
            ) => true,

            _ => false,
        }
    }

    pub fn is_any(&self) -> bool {
        match self {
            TypeRef::BuiltIn(BuiltInType::Any) => true,
            _ => false,
        }
    }

    pub fn is_nbt(&self) -> bool {
        match self {
            TypeRef::BuiltIn(BuiltInType::NBT | BuiltInType::Selector) => true,
            TypeRef::TypedNBT(_) => true,
            _ => false,
        }
    }

    pub fn is_stringy(&self) -> bool {
        match self {
            TypeRef::BuiltIn(
                BuiltInType::String
                | BuiltInType::Int
                | BuiltInType::Float
                | BuiltInType::Boolean
                | BuiltInType::Double,
            ) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            TypeRef::Array(_)
            | TypeRef::SizedArray(_, _)
            | TypeRef::BuiltIn(
                BuiltInType::Any
                | BuiltInType::Objective
                | BuiltInType::Selector
                | BuiltInType::Pos,
            ) => true,
            _ => false,
        }
    }
}

impl BuiltInType {
    pub fn is_numeric(&self) -> bool {
        match self {
            BuiltInType::Int | BuiltInType::Float | BuiltInType::Double => true,
            BuiltInType::Any => true, // This is technically true, since we don't check any.

            BuiltInType::Boolean
            | BuiltInType::String
            | BuiltInType::Time
            | BuiltInType::Identifier
            | BuiltInType::NBT
            | BuiltInType::Selector
            | BuiltInType::Pos
            | BuiltInType::Objective
            | BuiltInType::Void
            | BuiltInType::Transform => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum NBTSchema {
    /// A simple struct that validates based on the elements contained.
    Struct {
        name: String,
        elements: Vec<NBTSchemaElement>,
    },

    /// A varied struct, which is a struct that can be any one of the given variations.
    Varied {
        name: String,
        variations: Vec<NBTSchema>,
    },

    /// A union struct, which merges all the properties of the given schemas.
    Union { name: String, types: Vec<NBTSchema> },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NBTSchemaElement {
    pub name: String,
    pub value_type: TypeRef,
    pub required: bool,
}

#[macro_export]
macro_rules! nbt_schema_element {
    (builtin; [$req: expr] $name: expr => $ty: ident) => {
        $crate::dpscript::ty::NBTSchemaElement {
            required: $req,
            name: $name.into(),
            value_type: $crate::dpscript::ty::TypeRef::BuiltIn($crate::dpscript::ty::BuiltInType::$ty),
        }
    };

    ([req] builtin; $name: expr => $ty: ident) => {
        nbt_schema_element!(builtin; [true] $name => $ty)
    };

    (builtin; $name: expr => $ty: ident) => {
        nbt_schema_element!(builtin; [false] $name => $ty)
    };
}

pub mod schema {
    use crate::dpscript::ty::NBTSchema;
    use once_cell::sync::Lazy;

    pub const TEXT_COMPONENT: Lazy<NBTSchema> = Lazy::new(|| NBTSchema::Union {
        name: "Component".into(),
        types: vec![
            NBTSchema::Varied {
                name: "ComponentContents".into(),
                variations: vec![
                    NBTSchema::Struct {
                        name: "TextContents".into(),
                        elements: vec![nbt_schema_element!([req] builtin; "text" => String)],
                    },
                    NBTSchema::Struct {
                        name: "DataContents".into(),
                        elements: vec![
                            nbt_schema_element!([req] builtin; "storage" => String),
                            nbt_schema_element!([req] builtin; "path" => String),
                            nbt_schema_element!(builtin; "interpret" => Boolean),
                        ],
                    },
                    NBTSchema::Struct {
                        name: "SelectorContents".into(),
                        elements: vec![nbt_schema_element!([req] builtin; "selector" => String)],
                    },
                ],
            },
            NBTSchema::Struct {
                name: "ComponentStyle".into(),
                elements: vec![
                    nbt_schema_element!(builtin; "color" => String),
                    nbt_schema_element!(builtin; "bold" => Boolean),
                    nbt_schema_element!(builtin; "italic" => Boolean),
                    nbt_schema_element!(builtin; "strikethrough" => Boolean),
                    nbt_schema_element!(builtin; "underline" => Boolean),
                    // TODO: hover & click events
                ],
            },
        ],
    });
}
