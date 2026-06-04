use dpscript_analyzer::StaticRef;
use itertools::Itertools;
use miette::IntoDiagnostic;
use std::{collections::HashMap, fs};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

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
        "std/src/prelude.dps",
        "std/src/scoreboard.dps",
        "std/src/selectors.dps",
        "std/src/std.dps",
    ];

    let write_output =
        !std::env::args().contains("--no-output") && !std::env::args().contains("-N");

    let contents = files
        .iter()
        .map(|it| (*it, StaticRef::new(fs::read_to_string(it).unwrap())))
        .collect::<HashMap<_, _>>();

    let modules = dpscript_analyzer::analyze(&files, &contents, write_output)?;

    if write_output {
        std::fs::write(
            "modules.ron",
            ron::ser::to_string_pretty(&modules, ron::ser::PrettyConfig::new())
                .into_diagnostic()?,
        )
        .into_diagnostic()?;
    }

    for (_, v) in contents {
        v.free();
    }

    Ok(())
}
