use crate::{
    cx::VisitCx,
    err::{Error, Result},
    util::Either,
};
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
use dpscript_core::Spanned;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::{borrow::Borrow, collections::HashMap, hash::Hash, sync::Arc};

pub type RemoteGuard<'s, T> = MappedRwLockReadGuard<'s, Remote<T>>;
pub type RemoteGuardMut<'s, T> = MappedRwLockWriteGuard<'s, Remote<T>>;

pub trait MapReadGet<'s, K: Eq + Hash, T> {
    fn map_get<Q: Eq + Hash>(&'s self, key: &Q) -> Option<MappedRwLockReadGuard<'s, T>>
    where
        Q: ?Sized,
        K: Borrow<Q>;
}

pub trait MapReadGetMap<'s, K: Eq + Hash, T> {
    fn map_get<Q: Eq + Hash>(self, key: &Q) -> Option<MappedRwLockReadGuard<'s, T>>
    where
        Q: ?Sized,
        K: Borrow<Q>;
}

pub trait MapWriteGet<'s, K: Eq + Hash, T> {
    fn map_get_mut<Q: Eq + Hash>(&'s self, key: &Q) -> Option<MappedRwLockWriteGuard<'s, T>>
    where
        Q: ?Sized,
        K: Borrow<Q>;
}

pub trait MapWriteGetMap<'s, K: Eq + Hash, T> {
    fn map_get_mut<Q: Eq + Hash>(self, key: &Q) -> Option<MappedRwLockWriteGuard<'s, T>>
    where
        Q: ?Sized,
        K: Borrow<Q>;
}

impl<'s, K: Eq + Hash, T> MapReadGet<'s, K, T> for Arc<RwLock<HashMap<K, T>>> {
    fn map_get<Q: Eq + Hash>(&'s self, key: &Q) -> Option<MappedRwLockReadGuard<'s, T>>
    where
        Q: ?Sized,
        K: Borrow<Q>,
    {
        let guard = self.read();

        if guard.contains_key(key) {
            Some(RwLockReadGuard::map(guard, |v| v.get(key).unwrap()))
        } else {
            None
        }
    }
}

impl<'s, K: Eq + Hash, T> MapReadGetMap<'s, K, T> for MappedRwLockReadGuard<'s, HashMap<K, T>> {
    fn map_get<Q: Eq + Hash>(self, key: &Q) -> Option<MappedRwLockReadGuard<'s, T>>
    where
        Q: ?Sized,
        K: Borrow<Q>,
    {
        let guard = self;

        if guard.contains_key(key) {
            Some(MappedRwLockReadGuard::map(guard, |v| v.get(key).unwrap()))
        } else {
            None
        }
    }
}

impl<'s, K: Eq + Hash, T> MapWriteGet<'s, K, T> for Arc<RwLock<HashMap<K, T>>> {
    fn map_get_mut<Q: Eq + Hash>(&'s self, key: &Q) -> Option<MappedRwLockWriteGuard<'s, T>>
    where
        Q: ?Sized,
        K: Borrow<Q>,
    {
        let guard = self.write();

        if guard.contains_key(key) {
            Some(RwLockWriteGuard::map(guard, |v| v.get_mut(key).unwrap()))
        } else {
            None
        }
    }
}

impl<'s, K: Eq + Hash, T> MapWriteGetMap<'s, K, T> for MappedRwLockWriteGuard<'s, HashMap<K, T>> {
    fn map_get_mut<Q: Eq + Hash>(self, key: &Q) -> Option<MappedRwLockWriteGuard<'s, T>>
    where
        Q: ?Sized,
        K: Borrow<Q>,
    {
        let guard = self;

        if guard.contains_key(key) {
            Some(MappedRwLockWriteGuard::map(guard, |v| {
                v.get_mut(key).unwrap()
            }))
        } else {
            None
        }
    }
}

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
}

impl<'a, 'visit, 'view> VisitCx<'a, 'visit> {
    pub fn lookup(&self) -> ScopeLookup<'a, 'visit, '_> {
        ScopeLookup { cx: self }
    }

    pub fn lookup_mut(&self) -> ScopeLookupMut<'a, 'visit, '_> {
        ScopeLookupMut { cx: self }
    }
}

pub trait ScopeLookupTrait<'a: 'visit, 'visit: 's, 'view: 's, 's> {
    fn module_name(&self) -> String;
    fn cx(&'s self) -> &'s VisitCx<'a, 'visit>;

    // ================= TYPES =================

    fn lookup_type(&'s self, name: &str) -> Option<RemoteGuard<'s, TypeData<'a>>> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .find_map(move |s| s.types.map_get(name))
    }

    // ================= VAR OR CONST =================

    fn lookup_var_or_const(
        &'s self,
        name: Spanned<&str>,
    ) -> Result<Either<RemoteGuard<'s, Variable<'a>>, RemoteGuard<'s, Constant<'a>>>> {
        let cur_module = self.cx().module.name.clone();
        let var = self.lookup_var(name.0);
        let other = self.lookup_const(name.0);

        match (var, other) {
            (Some((var, _)), None) => Ok(Either::Left(var)),
            (None, Some((other, _))) => Ok(Either::Right(other)),

            (Some((var, var_depth)), Some((other, other_depth))) => {
                if var_depth > other_depth {
                    Ok(Either::Right(other))
                } else if var_depth < other_depth {
                    Ok(Either::Left(var))
                } else {
                    Err(Error::DuplicateDefs {
                        name: name.0.into(),
                        first: var.span.into(),
                        new: other.span.into(),
                        cur_module,
                    })
                }
            }

            (None, None) => Err(Error::UnresolvedName {
                name: name.0.into(),
                at: name.1.into(),
                cur_module,
            }),
        }
    }

    // ================= VARS =================

    /// Returns a variable node with the specified name and the depth at which it was found in the scope stack.
    fn lookup_var(&'s self, name: &str) -> Option<(RemoteGuard<'s, Variable<'a>>, usize)> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .enumerate()
            .find_map(|(i, s)| s.vars.map_get(name).map(|it| (it, i)))
    }

    // ================= CONSTS =================

    fn lookup_const(&'s self, name: &str) -> Option<(RemoteGuard<'s, Constant<'a>>, usize)> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .enumerate()
            .find_map(|(i, s)| s.consts.map_get(name).map(|it| (it, i)))
    }

    // ================= FUNCS =================

    fn lookup_func(&'s self, name: &str) -> Option<RemoteGuard<'s, FunctionInfo<'a>>> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .find_map(|s| s.funcs.map_get(name))
    }

    // ================= INSTANCE FUNCS =================

    fn lookup_inst_func(
        &'s self,
        name: &str,
        ty: &TypeRefId,
    ) -> Option<RemoteGuard<'s, FunctionInfo<'a>>> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .find_map(|s| s.inst_funcs.map_get(name).map(|m| m.map_get(ty)).flatten())
    }

    // ================= DSL FUNCS =================

    fn lookup_dsl_func(
        &'s self,
        ty: &TypeRefId,
        dsl: DslMarker,
    ) -> Option<RemoteGuard<'s, FunctionInfo<'a>>> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .find_map(|s| s.dsl_funcs.map_get(&dsl).map(|m| m.map_get(ty)).flatten())
    }

    // ================= OBJECTIVES =================

    fn lookup_objective(&'s self, name: &str) -> Option<RemoteGuard<'s, Objective<'a>>> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .find_map(|s| s.objectives.map_get(name))
    }
}

pub trait ScopeLookupMutTrait<'a: 'visit, 'visit: 's, 'view: 's, 's>:
    ScopeLookupTrait<'a, 'visit, 'view, 's>
{
    // ================= TYPES =================

    fn lookup_type_mut(&'s self, name: &'a str) -> Option<RemoteGuardMut<'s, TypeData<'a>>> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .find_map(|s| s.types.map_get_mut(name))
    }

    // ================= VAR OR CONST =================

    fn lookup_var_or_const_mut(
        &'s self,
        name: Spanned<&str>,
    ) -> Result<Either<RemoteGuardMut<'s, Variable<'a>>, RemoteGuardMut<'s, Constant<'a>>>> {
        // TODO: Optimize this shitty mess that does lookups multiple times

        let cur_module = self.module_name();
        let var_depth = self.lookup_var(name.0).map(|it| it.1).clone();
        let const_depth = self.lookup_const(name.0).map(|it| it.1).clone();

        if let Some(_var) = var_depth
            && const_depth.is_none()
        {
            Ok(Either::Left(self.lookup_var_mut(name.0).unwrap().0))
        } else if let Some(_other) = const_depth
            && var_depth.is_none()
        {
            Ok(Either::Right(self.lookup_const_mut(name.0).unwrap().0))
        } else if let Some(var_depth) = var_depth
            && let Some(const_depth) = const_depth
        {
            if var_depth > const_depth {
                Ok(Either::Right(self.lookup_const_mut(name.0).unwrap().0))
            } else if var_depth < const_depth {
                Ok(Either::Left(self.lookup_var_mut(name.0).unwrap().0))
            } else {
                let var_span = self.lookup_var_mut(name.0).unwrap().0.span;
                let const_span = self.lookup_const_mut(name.0).unwrap().0.span;

                Err(Error::DuplicateDefs {
                    name: name.0.into(),
                    first: var_span.into(),
                    new: const_span.into(),
                    cur_module,
                })
            }
        } else {
            Err(Error::UnresolvedName {
                name: name.0.into(),
                at: name.1.into(),
                cur_module,
            })
        }
    }

    // ================= VARS =================

    fn lookup_var_mut(&'s self, name: &str) -> Option<(RemoteGuardMut<'s, Variable<'a>>, usize)> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .enumerate()
            .find_map(|(i, s)| s.vars.map_get_mut(name).map(|it| (it, i)))
    }

    // ================= CONSTS =================

    fn lookup_const_mut(&'s self, name: &str) -> Option<(RemoteGuardMut<'s, Constant<'a>>, usize)> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .enumerate()
            .find_map(|(i, s)| s.consts.map_get_mut(name).map(|it| (it, i)))
    }

    // ================= FUNCS =================

    fn lookup_func_mut(&'s self, name: &str) -> Option<RemoteGuardMut<'s, FunctionInfo<'a>>> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .find_map(|s| s.funcs.map_get_mut(name))
    }

    // ================= INSTANCE FUNCS =================

    fn lookup_inst_func_mut(
        &'s self,
        name: &str,
        ty: &TypeRefId,
    ) -> Option<RemoteGuardMut<'s, FunctionInfo<'a>>> {
        self.cx().scope.stack.iter().rev().find_map(|s| {
            s.inst_funcs
                .map_get_mut(name)
                .map(|m| m.map_get_mut(ty))
                .flatten()
        })
    }

    // ================= DSL FUNCS =================

    fn lookup_dsl_func_mut(
        &'s self,
        ty: &TypeRefId,
        dsl: DslMarker,
    ) -> Option<RemoteGuardMut<'s, FunctionInfo<'a>>> {
        self.cx().scope.stack.iter().rev().find_map(|s| {
            s.dsl_funcs
                .map_get_mut(&dsl)
                .map(|m| m.map_get_mut(ty))
                .flatten()
        })
    }

    // ================= OBJECTIVES =================

    fn lookup_objective_mut(&'s self, name: &str) -> Option<RemoteGuardMut<'s, Objective<'a>>> {
        self.cx()
            .scope
            .stack
            .iter()
            .rev()
            .find_map(|s| s.objectives.map_get_mut(name))
    }
}

pub struct ScopeLookup<'a, 'visit, 'view> {
    cx: &'view VisitCx<'a, 'visit>,
}

impl<'a, 'visit: 's, 'view: 's, 's> ScopeLookupTrait<'a, 'visit, 'view, 's>
    for ScopeLookup<'a, 'visit, 'view>
{
    fn module_name(&self) -> String {
        self.cx.module.name.clone()
    }

    fn cx(&'s self) -> &'s VisitCx<'a, 'visit> {
        self.cx
    }
}

pub struct ScopeLookupMut<'a, 'visit, 'view> {
    cx: &'view VisitCx<'a, 'visit>,
}

impl<'a, 'visit: 's, 'view: 's, 's> ScopeLookupTrait<'a, 'visit, 'view, 's>
    for ScopeLookupMut<'a, 'visit, 'view>
{
    fn module_name(&self) -> String {
        self.cx.module.name.clone()
    }

    fn cx(&'s self) -> &'s VisitCx<'a, 'visit> {
        &*self.cx
    }
}

impl<'a, 'visit: 's, 'view: 's, 's> ScopeLookupMutTrait<'a, 'visit, 'view, 's>
    for ScopeLookupMut<'a, 'visit, 'view>
{
}
