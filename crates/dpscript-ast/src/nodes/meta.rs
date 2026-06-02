use dpscript_core::SourceSpan;

use crate::{
    prelude::{Spanned, types::TypeRef, value::literal::Literal},
    util::Name,
};

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet,
)]
pub enum DefFlags {
    Public,
    Const,
    Operator,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet,
)]
pub enum AllowFlag {
    /// Allow an incomplete declaration of this struct.
    Incomplete,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet, Default, HasSpan,
)]
pub struct DefMeta<'a> {
    /// Corresponds to `#[builtin]`.
    pub builtin: Option<BuiltinInfo<'a>>,

    /// Whether to inline the function.
    /// Only valid for a Function node.
    pub inline: bool,

    /// Flags to allow.
    pub allow: Vec<AllowFlag>,

    /// If this is a struct field, whether it can be null.
    pub nullable: bool,

    /// The hint ID.
    /// This is usually a registry, but can be anything, really.
    /// Used in the LSP for providing smart autocomplete for identifiers.
    pub hint: Option<Spanned<&'a str>>,

    /// If this is an item that can be used as a fill-in for a hint, this is its id.
    /// 
    /// Format: (hint, id)
    pub hint_id: Option<(Spanned<&'a str>, Spanned<&'a str>)>,

    /// Any `#[restrict(...)]` declarations.
    pub restrict: Vec<Restrict<'a>>,

    /// Any `#[require(...)]` declarations.
    pub require: Vec<Require<'a>>,

    /// Any `#[dsl(...)]` declarations.
    pub dsl: Vec<DslInfo<'a>>,

    /// If this is a command function, this is the template to build the command from.
    pub cmd: Option<Spanned<&'a str>>,

    /// A `#[since = ...]` annotation, used for libraries to show which version implemented
    /// this feature.
    pub since: Option<Spanned<&'a str>>,

    /// A `#[name = ...]` annotation, used to rename things internally to the user's choice.
    pub name: Option<Spanned<&'a str>>,

    /// Whether this argument is marked `#[this]`, marking it as the reciever in an instance function.
    pub this: bool,

    /// Any `#[enforce(...)]` declarations.
    pub enforce: Vec<EnforceType>,

    /// The way the object is represented.
    pub repr: Repr,

    /// This type can be converted into any of these other types by mapping field values directly.
    pub same_as: Vec<TypeRef<'a>>,

    /// The span of this metadata declaration.
    pub span: SourceSpan,

    /// Bounds to limit potential values to.
    /// 
    /// Only works on struct fields.
    pub limit: Option<(Literal<'a>, Literal<'a>)>,

    /// Whether this field is represented as raw JSON when serializing for commands.
    /// 
    /// Only works on struct fields.
    pub raw_json: bool,
}

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Facet, Default,
)]
pub enum Repr {
    #[default]
    Default,

    /// For structs: represented as an NBT object and defines its schema.
    Object,

    /// For structs: fields are represented and interpreted as an array in the order they are defined.
    Array,

    /// For enums: values are represented as the string provided in the definition.
    String,

    /// For enums: values are represented as the byte provided in the definition.
    Byte,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet)]
pub enum BuiltinInfo<'a> {
    /// `#[builtin(cast)]`
    ///
    /// The value will be cast by the compiler internally, even if it doesn't follow the schema.
    Cast,

    /// `#[builtin(convert_to = ...)]`
    ///
    /// The value will be converted by the compiler, ensuring it's convertable.
    ConvertTo(TypeRef<'a>),

    /// `#[builtin]`
    ///
    /// This function is implemented by the compiler.
    Generic,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet)]
pub enum Restrict<'a> {
    /// Restrict a field to a certain set of values.
    Values(Vec<Literal<'a>>),
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet)]
pub enum Require<'a> {
    /// Require one of the following fields in any initialization of the object.
    OneOf(Vec<Name<'a>>),

    /// Require the argument to be placed in data storage somewhere.
    Store,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet)]
pub enum DslInfo<'a> {
    /// This function can be a DSL if a value is prefixed with this value.
    Prefix(Spanned<&'a str>),
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Facet)]
pub enum EnforceType {
    /// `#[enforce(clone)]`
    /// Make sure the value is cloned before passing it into this function.
    Clone,
}
