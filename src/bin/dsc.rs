use clap::Parser;
use dpscript::cli::Cli;
use miette::highlighters::SyntectHighlighter;

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

    tracing_subscriber::fmt::init();
    Cli::parse().run()?;

    Ok(())
}
