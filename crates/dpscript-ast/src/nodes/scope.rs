use std::collections::HashMap;

use crate::{
    prelude::{
        def::{constant::Constant, func::FunctionInfo, objective::Objective},
        expr::var::Variable,
        types::{TypeData, TypeRefId},
        value::literal::DslMarker,
    },
    util::Remote,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Facet, Default)]
pub struct Scope<'a> {
    pub types: HashMap<&'a str, Remote<TypeData<'a>>>,
    pub vars: HashMap<&'a str, Remote<Variable<'a>>>,
    pub consts: HashMap<&'a str, Remote<Constant<'a>>>,
    pub funcs: HashMap<&'a str, Remote<FunctionInfo<'a>>>,

    // name -> target type -> func
    pub inst_funcs: HashMap<&'a str, HashMap<TypeRefId, Remote<FunctionInfo<'a>>>>,

    // dsl -> first arg type -> func
    pub dsl_funcs: HashMap<DslMarker, HashMap<TypeRefId, Remote<FunctionInfo<'a>>>>,

    pub objectives: HashMap<&'a str, Remote<Objective<'a>>>,
}
