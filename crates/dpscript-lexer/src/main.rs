use dpscript_lexer::parse;
use dpscript_parser::{tast_from_tokens, tokenize_first};
use miette::{Error, IntoDiagnostic, NamedSource};
use std::fs;
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
        // "std/src/prelude.dps",
        "std/src/scoreboard.dps",
        "std/src/selectors.dps",
        "std/src/std.dps",
    ];

    for file in files {
        tracing::info!("Work: {file}");

        let content = fs::read_to_string(&file).into_diagnostic()?;
        let tokens = tokenize_first(&file, &content)?;

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

        match parse(tokens) {
            Ok(defs) => {
                std::fs::write(
                    format!("{}.ron", file),
                    ron::ser::to_string_pretty(&defs, ron::ser::PrettyConfig::new())
                        .into_diagnostic()?,
                )
                .into_diagnostic()?;
            }

            Err(err) => {
                return Err(Error::new(err).with_source_code(NamedSource::new(file, content)));
            }
        }
    }

    Ok(())
}
