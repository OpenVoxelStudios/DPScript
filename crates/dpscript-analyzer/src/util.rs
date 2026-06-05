use dpscript_ast::{
    prelude::{
        SourceSpan,
        def::{constant::Constant, func::FunctionInfo, objective::Objective},
        types::TypeData,
    },
    util::Remote,
};
use dpscript_core::HasSpanGroup;
use facet::Facet;
use serde::Serialize;

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Facet, HasSpanGroup)]
pub enum Export<'a> {
    Constant(Remote<Constant<'a>>),
    Function(Remote<FunctionInfo<'a>>),
    Objective(Remote<Objective<'a>>),
    Type(Remote<TypeData<'a>>),
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
            Self::Constant(it) => Self::Constant(Remote {
                module,
                span,
                ..it.clone()
            }),

            Self::Function(it) => Self::Function(Remote {
                module,
                span,
                ..it.clone()
            }),

            Self::Objective(it) => Self::Objective(Remote {
                module,
                span,
                ..it.clone()
            }),

            Self::Type(it) => Self::Type(Remote {
                module,
                span,
                ..it.clone()
            }),
        }
    }
}
