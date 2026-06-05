use crate::{
    prelude::{SourceSpan, def::func::FunctionInfo, value::Value},
    util::{Name, Remote},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, HasSpan)]
pub struct Call<'a> {
    pub func: Name<'a>,
    pub target: Option<Box<Value<'a>>>,
    pub args: Vec<Value<'a>>,
    pub resolved: Option<Remote<FunctionInfo<'a>>>,
    pub span: SourceSpan,
}
