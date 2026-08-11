use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

use crate::{
    prelude::{
        def::{constant::Constant, func::FunctionInfo, objective::Objective},
        expr::var::Variable,
        types::{TypeData, TypeRefId},
        value::literal::DslMarker,
    },
    util::Remote,
};

pub type LockMap<K, V> = Arc<RwLock<HashMap<K, V>>>;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Scope<'a> {
    pub types: LockMap<&'a str, Remote<TypeData<'a>>>,
    pub vars: LockMap<&'a str, Remote<Variable<'a>>>,
    pub consts: LockMap<&'a str, Remote<Constant<'a>>>,
    pub funcs: LockMap<&'a str, Remote<FunctionInfo<'a>>>,

    // name -> target type -> func
    pub inst_funcs: LockMap<&'a str, HashMap<TypeRefId, Remote<FunctionInfo<'a>>>>,

    // dsl -> first arg type -> func
    pub dsl_funcs: LockMap<DslMarker, HashMap<TypeRefId, Remote<FunctionInfo<'a>>>>,

    pub objectives: LockMap<&'a str, Remote<Objective<'a>>>,
}
