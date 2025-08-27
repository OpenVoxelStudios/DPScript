#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TypeRef {
    /// A local (user-defined) type.
    Local(String),

    /// A type that is built-in to the language.
    BuiltIn(BuiltInType),

    /// A type for an NBT array.
    Array(Box<TypeRef>),

    /// A type for NBT data with a specific schema.
    /// If this should instead be unvalidated, use [`BuiltInType::NBT`].
    TypedNBT(NBTSchema),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BuiltInType {
    /// An integer type.
    Int,

    /// A float type.
    Float,

    /// A double type.
    Double,

    /// A true/false boolean type.
    Boolean,

    /// A time type.
    /// Examples:
    ///  - 5s
    ///  - 12m38s
    ///  - 14d
    Time,

    /// A type for any arbitrary string.
    String,

    /// A type for an identifier (or ResourceLocation).
    /// Should be a string in the form "namespace:path".
    Identifier,

    /// A type for arbitrary NBT (named binary tag) data.
    NBT,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
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
    Union {
        name: String,
        types: Vec<NBTSchema>,
    },
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
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
                        elements: vec![
                            nbt_schema_element!([req] builtin; "selector" => String),
                        ],
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
