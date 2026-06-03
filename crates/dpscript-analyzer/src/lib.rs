use crate::{
    cx::{AnalysisCx, Module},
    err::Error,
    passes::{
        exports::{ExportResolver, ExportStmtResolver},
        top_scope::TopScopeResolver,
    },
};
use dpscript_ast::prelude::{NamedSource, scope::Scope};
use dpscript_lexer::parse;
use dpscript_parser::{tast_from_tokens, tokenize_first};
use miette::Diagnostic;
use std::{collections::HashMap, mem::ManuallyDrop};
use thiserror::Error;

pub mod cx;
pub mod err;
pub mod passes;
pub mod util;
pub mod visitor;

mod prelude {
    pub use dpscript_ast::prelude::*;
}

#[derive(Debug, Error, Diagnostic)]
#[error("module errors emitted")]
#[diagnostic()]
pub struct ModuleErrors {
    #[source_code]
    pub code: miette::NamedSource<String>,

    #[related]
    pub inner: Vec<Error>,
}

#[derive(Debug, Error, Diagnostic)]
#[error("errors emitted")]
#[diagnostic()]
pub struct ErrorGroup {
    #[related]
    pub inner: Vec<ModuleErrors>,
}

pub struct StaticRef {
    owned: ManuallyDrop<String>,
    ptr: *const str,
}

impl StaticRef {
    pub fn new(owned: String) -> Self {
        Self {
            ptr: owned.as_str() as *const str,
            owned: ManuallyDrop::new(owned),
        }
    }

    pub fn get<'t>(&self) -> &'t str {
        unsafe { &*self.ptr }
    }

    pub fn free(mut self) {
        unsafe { ManuallyDrop::drop(&mut self.owned) };
    }
}

pub fn analyze<'a>(
    files: &[&'static str],
    contents: &HashMap<&'static str, StaticRef>,
) -> Result<HashMap<String, Module<'a>>, ErrorGroup> {
    let mut modules = HashMap::new();

    for file in files {
        tracing::info!("Work: {file}");

        let mut parts = file.trim_end_matches(".dps").split("/").collect::<Vec<_>>();

        parts.remove(1);

        if parts.last() == parts.get(parts.len() - 2) {
            parts.pop();
        }

        let name = parts.join("::");
        let content = contents.get(file).unwrap();
        let text = content.get();
        let tokens = tokenize_first(&file, text).unwrap();

        std::fs::write(
            format!("{}.tokens_raw.ron", file),
            ron::ser::to_string_pretty(&tokens, ron::ser::PrettyConfig::new()).unwrap(),
        )
        .unwrap();

        let tokens = tast_from_tokens(tokens).unwrap();

        std::fs::write(
            format!("{}.tokens.ron", file),
            ron::ser::to_string_pretty(&tokens, ron::ser::PrettyConfig::new()).unwrap(),
        )
        .unwrap();

        let defs = match parse(tokens) {
            Ok(defs) => {
                std::fs::write(
                    format!("{}.ron", file),
                    ron::ser::to_string_pretty(&defs, ron::ser::PrettyConfig::new()).unwrap(),
                )
                .unwrap();

                defs
            }

            Err(err) => {
                panic!("{err}");
            }
        };

        let module = Module {
            defs,
            exports: HashMap::new(),
            inst_func_exports: HashMap::new(),
            name: name.clone(),
            scope: Scope::default(),
            source: NamedSource { file, code: text },
        };

        modules.insert(name, module);
    }

    let mut cx = AnalysisCx::new(modules);

    cx.run_pass(&mut ExportResolver);
    cx.run_pass(&mut ExportStmtResolver);
    cx.run_pass(&mut TopScopeResolver);

    let modules = cx.modules;

    if !cx.errors.is_empty() {
        let mut errs = HashMap::<String, ModuleErrors>::new();

        for err in cx.errors {
            let module = err.module();

            errs.entry(module.clone())
                .or_insert_with(|| ModuleErrors {
                    code: modules.get(&module).unwrap().source.clone().into(),
                    inner: Vec::new(),
                })
                .inner
                .push(err);
        }

        return Err(ErrorGroup {
            inner: errs.into_values().collect(),
        });
    }

    Ok(modules)
}
