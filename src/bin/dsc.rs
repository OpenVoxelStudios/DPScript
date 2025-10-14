use clap::Parser;
use dpscript::cli::Cli;
use miette::highlighters::SyntectHighlighter;
use tracing::level_filters::LevelFilter;
use tracing_indicatif::{filter::hide_indicatif_span_fields, IndicatifLayer};
use tracing_subscriber::{fmt::format::DefaultFields, layer::SubscriberExt, EnvFilter};

pub fn main() -> miette::Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(3)
                .tab_width(4)
                .break_words(true)
                .color(true)
                .with_cause_chain()
                .with_syntax_highlighting(SyntectHighlighter::default())
                .build(),
        )
    }))?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        ))
        .with(IndicatifLayer::new().with_span_field_formatter(hide_indicatif_span_fields(DefaultFields::new())))
        .init();

    Cli::parse().run()?;

    Ok(())
}
