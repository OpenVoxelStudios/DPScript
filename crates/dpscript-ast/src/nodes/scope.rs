use std::collections::HashMap;

use crate::{
    prelude::{
        def::func::FunctionInfo,
        types::{TypeData, TypeRef},
    },
    util::{Name, Remote},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, Default)]
pub struct Scope<'a> {
    pub types: HashMap<&'a str, Remote<TypeData<'a>>>,
    pub vars: HashMap<&'a str, TypeRef<'a>>,
    pub consts: HashMap<&'a str, TypeRef<'a>>,
    pub funcs: HashMap<&'a str, FunctionInfo<'a>>,
    // It's dumb but I have to use to_string() to make the TypeRefData something we can use as a key
    pub inst_funcs: HashMap<&'a str, HashMap<String, FunctionInfo<'a>>>,
    pub objectives: HashMap<&'a str, Name<'a>>,
}
