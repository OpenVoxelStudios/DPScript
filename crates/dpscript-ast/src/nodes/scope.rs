use std::collections::HashMap;

use crate::prelude::{
    def::func::FunctionInfo,
    types::{TypeData, TypeRef},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, Default)]
pub struct Scope<'a> {
    pub types: HashMap<&'a str, TypeData<'a>>,
    pub vars: HashMap<&'a str, TypeRef<'a>>,
    pub consts: HashMap<&'a str, TypeRef<'a>>,
    pub funcs: HashMap<&'a str, FunctionInfo<'a>>,
}
