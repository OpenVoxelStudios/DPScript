use dpscript_analyzer::{
    cx::{AnalysisCx, Module},
    passes::exports::ExportResolver,
};
use dpscript_ast::prelude::scope::Scope;
use dpscript_lexer::parse;
use dpscript_parser::{tast_from_tokens, tokenize_first};
use miette::{Error, IntoDiagnostic, NamedSource};
use std::{collections::HashMap, fs, mem::ManuallyDrop};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

struct StaticRef {
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

pub fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(LevelFilter::INFO)
        .with_span_events(FmtSpan::FULL)
        .init();

    let files = [
        "std/src/gm/gm.dps",
        "std/src/gm/simple.dps",
        "std/src/gm/sqrt.dps",
        "std/src/types/entities/display.dps",
        "std/src/types/entities/entities.dps",
        "std/src/types/base.dps",
        "std/src/types/blocks.dps",
        "std/src/types/items.dps",
        "std/src/types/level.dps",
        "std/src/types/math.dps",
        "std/src/types/misc.dps",
        "std/src/types/players.dps",
        "std/src/types/text.dps",
        "std/src/types/transform.dps",
        "std/src/types/types.dps",
        "std/src/base.dps",
        "std/src/entity.dps",
        "std/src/intrinsics.dps",
        "std/src/math.dps",
        // "std/src/prelude.dps",
        "std/src/scoreboard.dps",
        "std/src/selectors.dps",
        "std/src/std.dps",
    ];

    let mut modules = HashMap::new();

    let contents = files
        .iter()
        .map(|it| (*it, StaticRef::new(fs::read_to_string(it).unwrap())))
        .collect::<HashMap<_, _>>();

    for file in files {
        tracing::info!("Work: {file}");

        let name = file
            .trim_end_matches(".dps")
            .replace("src/", "")
            .replace("/", "::");

        let content = contents.get(file).unwrap();
        let text = content.get();
        let tokens = tokenize_first(&file, text)?;

        std::fs::write(
            format!("{}.tokens_raw.ron", file),
            ron::ser::to_string_pretty(&tokens, ron::ser::PrettyConfig::new()).into_diagnostic()?,
        )
        .into_diagnostic()?;

        let tokens = tast_from_tokens(tokens)?;

        std::fs::write(
            format!("{}.tokens.ron", file),
            ron::ser::to_string_pretty(&tokens, ron::ser::PrettyConfig::new()).into_diagnostic()?,
        )
        .into_diagnostic()?;

        let defs = match parse(tokens) {
            Ok(defs) => {
                std::fs::write(
                    format!("{}.ron", file),
                    ron::ser::to_string_pretty(&defs, ron::ser::PrettyConfig::new())
                        .into_diagnostic()?,
                )
                .into_diagnostic()?;

                defs
            }

            Err(err) => {
                return Err(Error::new(err).with_source_code(NamedSource::new(file, content.get())));
            }
        };

        let module = Module {
            defs,
            exports: HashMap::new(),
            name: name.clone(),
            scope: Scope::default(),
            source: text,
        };

        modules.insert(name, module);
    }

    let mut analyzer = AnalysisCx::new(modules);

    analyzer.run_pass(&mut ExportResolver);

    std::fs::write(
        "modules.ron",
        ron::ser::to_string_pretty(&analyzer.modules, ron::ser::PrettyConfig::new())
            .into_diagnostic()?,
    )
    .into_diagnostic()?;

    for (_, v) in contents {
        v.free();
    }

    Ok(())
}
