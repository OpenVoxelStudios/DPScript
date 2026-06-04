use dpscript_ast::prelude::{NamedSource, def::Def, scope::Scope};
use facet::Facet;
use itertools::Itertools;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

use crate::{
    err::{Error, Warning},
    util::{Export, FuncExport},
    visitor::DefVisitor,
};

pub struct VisitCx<'a, 'visit> {
    pub scope: ScopeCx<'a>,
    pub module: Module<'a>,
    pub analysis: &'visit mut AnalysisCx<'a>,
}

pub struct ScopeCx<'a> {
    pub stack: Vec<Scope<'a>>,
}

impl<'a> ScopeCx<'a> {
    pub fn current(&mut self) -> &mut Scope<'a> {
        self.stack.last_mut().unwrap()
    }
}

#[derive(Default, Serialize, Facet)]
pub struct Module<'a> {
    pub name: String,
    pub exports: HashMap<&'a str, Export<'a>>,
    pub inst_func_exports: HashMap<&'a str, HashMap<String, FuncExport<'a>>>,
    pub defs: Vec<Def<'a>>,
    pub scope: Scope<'a>,
    pub source: NamedSource<'a>,
}

#[derive(Default)]
pub struct AnalysisCx<'a> {
    pub errors: Vec<Error>,
    pub warnings: Vec<Warning>,

    pub modules: HashMap<String, Module<'a>>,
    pub visited_modules: HashSet<String>,
}

impl<'a> AnalysisCx<'a> {
    pub fn new(modules: HashMap<String, Module<'a>>) -> Self {
        Self {
            modules,
            ..Default::default()
        }
    }

    pub fn err(&mut self, err: impl Into<Error>) {
        self.errors.push(err.into());
    }

    pub fn warn(&mut self, warn: impl Into<Warning>) {
        self.warnings.push(warn.into());
    }

    pub fn run_pass<T>(&mut self, visitor: &mut T)
    where
        for<'visit> T: DefVisitor<'a, 'visit>,
    {
        self.visited_modules.clear();

        let keys = self.modules.keys().cloned().sorted().collect::<Vec<_>>();

        for key in keys {
            self.visit_module(key, visitor);
        }

        self.visited_modules.clear();
    }

    pub fn visit_module<'visit, T: DefVisitor<'a, 'visit>>(
        &'visit mut self,
        module_id: impl AsRef<str>,
        visitor: &mut T,
    ) -> bool
    where
        'a: 'visit,
    {
        if self.visited_modules.contains(module_id.as_ref()) {
            return true;
        }

        self.visited_modules.insert(module_id.as_ref().into());

        if self.modules.contains_key(module_id.as_ref()) {
            let mut module_tmp = self
                .modules
                .insert(module_id.as_ref().into(), Default::default())
                .unwrap();

            let mut defs = core::mem::take(&mut module_tmp.defs);

            let mut cx = VisitCx {
                scope: ScopeCx {
                    stack: vec![core::mem::take(&mut module_tmp.scope)],
                },

                analysis: self,
                module: module_tmp,
            };

            for item in &mut defs {
                visitor.visit_def(&mut cx, item);
            }

            if cx.scope.stack.len() != 1 {
                panic!("Multiple scopes on stack, expected only one!");
            }

            cx.module.defs = defs;
            cx.module.scope = cx.scope.stack.pop().unwrap();

            cx.analysis
                .modules
                .insert(module_id.as_ref().into(), core::mem::take(&mut cx.module));

            true
        } else {
            false
        }
    }
}
