use dpscript_ast::{
    prelude::{
        SourceSpan,
        def::{enums::EnumVariant, func::FunctionInfo, structs::StructField},
        meta::DefMeta,
        types::TypeRef,
    },
    util::Name,
};
use dpscript_core::{HasSpan, HasSpanGroup};
use facet::Facet;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Facet, HasSpan)]
pub struct ConstExport<'a> {
    pub name: Name<'a>,
    pub ty: TypeRef<'a>,
    pub meta: DefMeta<'a>,
    pub span: SourceSpan,
    pub module: String,
}

#[derive(Debug, Clone, Serialize, Facet, HasSpan)]
pub struct FuncExport<'a> {
    pub info: FunctionInfo<'a>,
    pub meta: DefMeta<'a>,
    pub span: SourceSpan,
    pub module: String,
}

#[derive(Debug, Clone, Serialize, Facet, HasSpan)]
pub struct ObjectiveExport<'a> {
    pub name: Name<'a>,
    pub meta: DefMeta<'a>,
    pub span: SourceSpan,
    pub module: String,
}

#[derive(Debug, Clone, Serialize, Facet, HasSpan)]
pub struct StructExport<'a> {
    pub name: Name<'a>,
    pub meta: DefMeta<'a>,
    pub extends: Vec<TypeRef<'a>>,
    pub fields: Vec<StructField<'a>>,
    pub span: SourceSpan,
    pub module: String,
}

#[derive(Debug, Clone, Serialize, Facet, HasSpan)]
pub struct EnumExport<'a> {
    pub name: Name<'a>,
    pub meta: DefMeta<'a>,
    pub variants: Vec<EnumVariant<'a>>,
    pub span: SourceSpan,
    pub module: String,
}

#[derive(Debug, Clone, Serialize, Facet, HasSpan)]
pub struct TypedefExport<'a> {
    pub name: Name<'a>,
    pub meta: DefMeta<'a>,
    pub span: SourceSpan,
    pub module: String,
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Facet, HasSpanGroup)]
pub enum Export<'a> {
    Constant(ConstExport<'a>),
    Function(FuncExport<'a>),
    Objective(ObjectiveExport<'a>),
    Struct(StructExport<'a>),
    Enum(EnumExport<'a>),
    Typedef(TypedefExport<'a>),
}

impl<'a> Export<'a> {
    pub fn module(&self) -> &str {
        match self {
            Self::Constant(it) => &it.module,
            Self::Function(it) => &it.module,
            Self::Objective(it) => &it.module,
            Self::Struct(it) => &it.module,
            Self::Enum(it) => &it.module,
            Self::Typedef(it) => &it.module,
        }
    }

    pub fn with(&self, span: SourceSpan, module: impl AsRef<str>) -> Self {
        let module = module.as_ref().into();

        match self {
            Self::Constant(it) => Self::Constant(ConstExport {
                module,
                span,
                ..it.clone()
            }),

            Self::Function(it) => Self::Function(FuncExport {
                module,
                span,
                ..it.clone()
            }),

            Self::Objective(it) => Self::Objective(ObjectiveExport {
                module,
                span,
                ..it.clone()
            }),

            Self::Struct(it) => Self::Struct(StructExport {
                module,
                span,
                ..it.clone()
            }),

            Self::Enum(it) => Self::Enum(EnumExport {
                module,
                span,
                ..it.clone()
            }),

            Self::Typedef(it) => Self::Typedef(TypedefExport {
                module,
                span,
                ..it.clone()
            }),
        }
    }
}
