use crate::{
    cx::{AnalysisCx, Module},
    err::Error,
    passes::{
        basic_exports::BasicExportResolver,
        basic_scope::BasicScopeResolver,
        exports::{ExportResolver, ExportStmtResolver},
        inference::TypeInference,
        lowering::AstLowering,
        resolution::TypeResolver,
        top_scope::TopScopeResolver,
    },
    refs::StaticRef,
};
use dpscript_ast::prelude::{NamedSource, scope::Scope};
use dpscript_lexer::parse;
use dpscript_parser::{tast_from_tokens, tokenize_first};
use miette::Diagnostic;
use std::collections::HashMap;
use thiserror::Error;

pub mod checks;
pub mod cx;
pub mod err;
pub mod ops;
pub mod passes;
pub mod refs;
pub mod scope;
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
#[error("{count} errors emitted")]
#[diagnostic()]
pub struct ErrorGroup {
    #[related]
    pub inner: Vec<ModuleErrors>,
    pub count: usize,
}

macro_rules! passes {
    {
        $cx: ident;

        $($ty: ident = $name: expr),*
        $(,)?
    } => {
        let len = [$($name),*].len() as u64;
        let pb = indicatif::ProgressBar::new(len)
            .with_style(
                indicatif::ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] [{pos}/{len}] {msg:.green.bold}")
                    .unwrap()
                    .progress_chars("##-")
            );

        $(
            pb.set_message($name);
            $cx.run_pass(&mut $ty);
            pb.inc(1);
        )*

        pb.finish();
    };
}

pub fn analyze<'a>(
    files: &[&'static str],
    contents: &HashMap<&'static str, StaticRef>,
    write_output: bool,
) -> Result<HashMap<String, Module<'a>>, (HashMap<String, Module<'a>>, ErrorGroup)> {
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

        if write_output {
            std::fs::write(
                format!("{}.tokens_raw.ron", file),
                ron::ser::to_string_pretty(&tokens, ron::ser::PrettyConfig::new()).unwrap(),
            )
            .unwrap();
        }

        let tokens = tast_from_tokens(tokens).unwrap();

        if write_output {
            std::fs::write(
                format!("{}.tokens.ron", file),
                ron::ser::to_string_pretty(&tokens, ron::ser::PrettyConfig::new()).unwrap(),
            )
            .unwrap();
        }

        let defs = match parse(tokens) {
            Ok(defs) => {
                if write_output {
                    std::fs::write(
                        format!("{}.ron", file),
                        ron::ser::to_string_pretty(&defs, ron::ser::PrettyConfig::new()).unwrap(),
                    )
                    .unwrap();
                }

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

    passes! {
        cx;

        AstLowering = "AST Lowering",
        BasicExportResolver = "Basic Exports",
        BasicScopeResolver = "Basic Scopes",
        TypeResolver = "Type Resolution (#1)",
        ExportResolver = "Module Exports",
        ExportStmtResolver = "Export Statements",
        TopScopeResolver = "Scope Resolution",
        TypeResolver = "Type Resolution (#2)",
        TypeInference = "Type Inference",
    };

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

        return Err((
            modules,
            ErrorGroup {
                count: errs.values().map(|it| it.inner.len()).sum(),
                inner: errs.into_values().collect(),
            },
        ));
    }

    Ok(modules)
}
