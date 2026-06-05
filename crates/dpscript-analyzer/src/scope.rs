use dpscript_ast::{
    prelude::{
        def::{constant::Constant, func::FunctionInfo, objective::Objective},
        expr::var::Variable,
        scope::Scope,
        types::{TypeData, TypeRefId},
        value::literal::DslMarker,
    },
    util::Remote,
};

pub struct ScopeCx<'a> {
    pub stack: Vec<Scope<'a>>,
}

impl<'a> ScopeCx<'a> {
    pub fn current(&mut self) -> &mut Scope<'a> {
        self.stack.last_mut().unwrap()
    }

    pub fn push(&mut self) -> &mut Scope<'a> {
        self.stack.push(Scope::default());
        self.current()
    }

    pub fn push_existing(&mut self, scope: Scope<'a>) -> &mut Scope<'a> {
        self.stack.push(scope);
        self.current()
    }

    pub fn pop(&mut self) -> Scope<'a> {
        self.stack.pop().expect("Not enough scopes on the stack!")
    }

    pub fn lookup(&mut self) -> ScopeLookup<'a, '_> {
        ScopeLookup { cx: self }
    }
}

pub struct ScopeLookup<'a, 'view> {
    cx: &'view mut ScopeCx<'a>,
}

impl<'a, 'view> ScopeLookup<'a, 'view> {
    // ================= TYPES =================

    pub fn lookup_type<'r>(&'r self, name: &str) -> Option<&'r Remote<TypeData<'a>>> {
        self.cx.stack.iter().rev().find_map(|s| s.types.get(name))
    }

    pub fn lookup_type_mut<'r>(
        &'r mut self,
        name: &'a str,
    ) -> Option<&'r mut Remote<TypeData<'a>>> {
        self.cx
            .stack
            .iter_mut()
            .rev()
            .find_map(|s| s.types.get_mut(name))
    }

    // ================= VARS =================

    pub fn lookup_var<'r>(&'r self, name: &str) -> Option<&'r Remote<Variable<'a>>> {
        self.cx.stack.iter().rev().find_map(|s| s.vars.get(name))
    }

    pub fn lookup_var_mut<'r>(&'r mut self, name: &str) -> Option<&'r mut Remote<Variable<'a>>> {
        self.cx
            .stack
            .iter_mut()
            .rev()
            .find_map(|s| s.vars.get_mut(name))
    }

    // ================= CONSTS =================

    pub fn lookup_const<'r>(&'r self, name: &str) -> Option<&'r Remote<Constant<'a>>> {
        self.cx.stack.iter().rev().find_map(|s| s.consts.get(name))
    }

    pub fn lookup_const_mut<'r>(&'r mut self, name: &str) -> Option<&'r mut Remote<Constant<'a>>> {
        self.cx
            .stack
            .iter_mut()
            .rev()
            .find_map(|s| s.consts.get_mut(name))
    }

    // ================= FUNCS =================

    pub fn lookup_func<'r>(&'r self, name: &str) -> Option<&'r Remote<FunctionInfo<'a>>> {
        self.cx.stack.iter().rev().find_map(|s| s.funcs.get(name))
    }

    pub fn lookup_func_mut<'r>(
        &'r mut self,
        name: &str,
    ) -> Option<&'r mut Remote<FunctionInfo<'a>>> {
        self.cx
            .stack
            .iter_mut()
            .rev()
            .find_map(|s| s.funcs.get_mut(name))
    }

    // ================= INSTANCE FUNCS =================

    pub fn lookup_inst_func<'r>(
        &'r self,
        name: &str,
        ty: &TypeRefId,
    ) -> Option<&'r Remote<FunctionInfo<'a>>> {
        self.cx
            .stack
            .iter()
            .rev()
            .find_map(|s| s.inst_funcs.get(name).map(|m| m.get(ty)).flatten())
    }

    pub fn lookup_inst_func_mut<'r>(
        &'r mut self,
        name: &str,
        ty: &TypeRefId,
    ) -> Option<&'r mut Remote<FunctionInfo<'a>>> {
        self.cx
            .stack
            .iter_mut()
            .rev()
            .find_map(|s| s.inst_funcs.get_mut(name).map(|m| m.get_mut(ty)).flatten())
    }

    // ================= DSL FUNCS =================

    pub fn lookup_dsl_func<'r>(
        &'r self,
        ty: &TypeRefId,
        dsl: DslMarker,
    ) -> Option<&'r Remote<FunctionInfo<'a>>> {
        self.cx
            .stack
            .iter()
            .rev()
            .find_map(|s| s.dsl_funcs.get(&dsl).map(|m| m.get(ty)).flatten())
    }

    pub fn lookup_dsl_func_mut<'r>(
        &'r mut self,
        ty: &TypeRefId,
        dsl: DslMarker,
    ) -> Option<&'r mut Remote<FunctionInfo<'a>>> {
        self.cx
            .stack
            .iter_mut()
            .rev()
            .find_map(|s| s.dsl_funcs.get_mut(&dsl).map(|m| m.get_mut(ty)).flatten())
    }

    // ================= OBJECTIVES =================

    pub fn lookup_objective<'r>(&'r self, name: &str) -> Option<&'r Remote<Objective<'a>>> {
        self.cx
            .stack
            .iter()
            .rev()
            .find_map(|s| s.objectives.get(name))
    }

    pub fn lookup_objective_mut<'r>(
        &'r mut self,
        name: &str,
    ) -> Option<&'r mut Remote<Objective<'a>>> {
        self.cx
            .stack
            .iter_mut()
            .rev()
            .find_map(|s| s.objectives.get_mut(name))
    }
}
