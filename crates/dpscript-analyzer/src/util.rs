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

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left(_))
    }

    pub fn is_right(&self) -> bool {
        matches!(self, Self::Right(_))
    }

    pub fn as_left(&self) -> Option<&A> {
        match self {
            Self::Left(it) => Some(it),
            Self::Right(_) => None,
        }
    }

    pub fn as_right(&self) -> Option<&B> {
        match self {
            Self::Right(it) => Some(it),
            Self::Left(_) => None,
        }
    }

    pub fn as_left_mut(&mut self) -> Option<&mut A> {
        match self {
            Self::Left(it) => Some(it),
            Self::Right(_) => None,
        }
    }

    pub fn as_right_mut(&mut self) -> Option<&mut B> {
        match self {
            Self::Right(it) => Some(it),
            Self::Left(_) => None,
        }
    }

    pub fn into_left(self) -> Option<A> {
        match self {
            Self::Left(it) => Some(it),
            Self::Right(_) => None,
        }
    }

    pub fn into_right(self) -> Option<B> {
        match self {
            Self::Right(it) => Some(it),
            Self::Left(_) => None,
        }
    }

    pub fn unwrap_left(self) -> A {
        self.into_left().unwrap()
    }

    pub fn unwrap_right(self) -> B {
        self.into_right().unwrap()
    }
}

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
