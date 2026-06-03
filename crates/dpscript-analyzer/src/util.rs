use dpscript_ast::{
    prelude::{
        SourceSpan,
        def::func::FunctionInfo,
        meta::DefMeta,
        types::{TypeData, TypeRef},
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
pub struct TypeExport<'a> {
    pub name: Name<'a>,
    pub data: TypeData<'a>,
    pub span: SourceSpan,
    pub module: String,
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Facet, HasSpanGroup)]
pub enum Export<'a> {
    Constant(ConstExport<'a>),
    Function(FuncExport<'a>),
    Objective(ObjectiveExport<'a>),
    Type(TypeExport<'a>),
}

impl<'a> Export<'a> {
    pub fn module(&self) -> &str {
        match self {
            Self::Constant(it) => &it.module,
            Self::Function(it) => &it.module,
            Self::Objective(it) => &it.module,
            Self::Type(it) => &it.module,
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

            Self::Type(it) => Self::Type(TypeExport {
                module,
                span,
                ..it.clone()
            }),
        }
    }
}
