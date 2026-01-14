#![allow(unused)]

use backtrace::Backtrace;
use miette::{
    Diagnostic, GraphicalTheme, NamedSource, Report, SourceOffset, SourceSpan, ThemeCharacters,
    ThemeStyles,
};
use owo_colors::Style;
use std::{
    fmt::Write,
    fs::File,
    io::{self, Read},
    panic::Location,
    path::Path,
    sync::Once,
};
use thiserror::Error;

static INSTALLER: Once = Once::new();

const HELP_TEXT: &str = "run with `RUST_BACKTRACE=1` environment variable to display a backtrace";

pub fn is_rust_backtrace_enabled() -> bool {
    if let Ok(var) = std::env::var("RUST_BACKTRACE") {
        !var.is_empty() && var != "0"
    } else {
        false
    }
}

pub fn format_backtrace() -> String {
    // This is all taken from human-panic: https://github.com/rust-cli/human-panic/blob/master/src/report.rs#L55-L107
    const HEX_WIDTH: usize = std::mem::size_of::<usize>() + 2;
    
    //Padding for next lines after frame's address
    const NEXT_SYMBOL_PADDING: usize = HEX_WIDTH + 6;
    
    let mut backtrace = String::from("\n==== Backtrace ====\n");

    for (idx, frame) in Backtrace::new().frames().iter().skip(10).enumerate() {
        let ip = frame.ip();
        let _ = write!(backtrace, "\n{:4}: {:2$?}", idx, ip, HEX_WIDTH);
        let symbols = frame.symbols();

        if symbols.is_empty() {
            let _ = write!(backtrace, " - <unresolved>");
            continue;
        }

        for (idx, symbol) in symbols.iter().enumerate() {
            //Print symbols from this address,
            //if there are several addresses
            //we need to put it on next line
            if idx != 0 {
                let _ = write!(backtrace, "\n{:1$}", "", NEXT_SYMBOL_PADDING);
            }

            if let Some(name) = symbol.name() {
                let _ = write!(backtrace, " - {}", name);
            } else {
                let _ = write!(backtrace, " - <unknown>");
            }

            //See if there is debug information with file name and line
            if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                let _ = write!(
                    backtrace,
                    "\n{:3$}at {}:{}",
                    "",
                    file.display(),
                    line,
                    NEXT_SYMBOL_PADDING
                );
            }
        }
    }

    backtrace
}

#[derive(Clone, Debug, Error, Diagnostic)]
#[error("{message}{}", .backtrace.clone().unwrap_or_default())]
#[diagnostic()]
pub struct Panic {
    message: String,
    backtrace: Option<String>,
    #[help]
    help: Option<&'static str>,
}

impl Panic {
    pub fn new(message: String) -> Self {
        let (help, backtrace) = if is_rust_backtrace_enabled() {
            (None, Some(format_backtrace()))
        } else {
            (Some(HELP_TEXT), None)
        };

        Self {
            message,
            help,
            backtrace,
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("Panic at {}:{}:{}", filename, line, col)]
#[diagnostic()]
pub struct PanicLocation {
    #[source]
    #[diagnostic_source]
    panic: Panic,
    filename: String,
    line: u32,
    col: u32,
    #[help]
    help: Option<&'static str>,
}

impl PanicLocation {
    fn new(panic: Panic, location: &Location) -> Self {
        Self {
            panic,
            filename: location.file().to_string(),
            line: location.line(),
            col: location.column(),

            help: if is_rust_backtrace_enabled() {
                None
            } else {
                Some(HELP_TEXT)
            },
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("Error at {}:{}:{}", filename, line, col)]
#[diagnostic()]
/// A miette report for test case source code snippets
pub struct TestSourceSpan {
    filename: String,
    line: usize,
    col: usize,
    #[source_code]
    src: NamedSource<String>,
    #[label("here")]
    span: SourceSpan,
    #[related]
    related: Vec<Report>,
}

impl TestSourceSpan {
    /// Fetch miette source code and source span from given filename and line
    pub fn from_location(
        filename: String,
        line: usize,
        col: usize,
    ) -> std::io::Result<TestSourceSpan> {
        let mut file = File::open(&filename)?;
        let mut source = String::new();

        file.read_to_string(&mut source)?;

        let start_offset = SourceOffset::from_location(&source, line, col).offset();

        // find byte offset at end of line
        let end_offset = source[start_offset..]
            .lines()
            .next()
            .map(|line| start_offset + line.trim_end().len())
            .unwrap_or_else(|| source.trim_end().len());

        let span = (start_offset..end_offset).into();

        Ok(TestSourceSpan {
            src: NamedSource::new(&filename, source),
            span,
            filename,
            line,
            col,
            related: Vec::new(),
        })
    }

    /// Add an error to the list of related errors
    pub fn add_related<E: Into<Report>>(&mut self, err: E) {
        self.related.push(err.into());
    }

    /// Tries to find source information from backtrace.
    pub fn from_backtrace(caller: &Location) -> io::Result<Option<Self>> {
        // A substring of test source file paths
        let test_path = Path::new(caller.file()).parent().unwrap().to_string_lossy();

        for frame in Backtrace::new().frames().iter() {
            for symbol in frame.symbols().iter() {
                if let Some(filename) = symbol.filename().and_then(|f| f.to_str()) {
                    if filename.contains(test_path.as_ref()) {
                        if let (Some(line), Some(col)) = (symbol.lineno(), symbol.colno()) {
                            return Ok(Some(Self::from_location(
                                filename.into(),
                                line as usize,
                                col as usize,
                            )?));
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

#[track_caller]
pub fn set_panic_hook() {
    set_panic_hook_with_caller(Location::caller())
}

pub fn set_panic_hook_with_caller(caller: &'static Location) {
    std::panic::set_hook(Box::new(move |info| {
        let payload = info.payload();

        let message = if let Some(msg) = payload.downcast_ref::<&str>() {
            msg.to_string()
        } else if let Some(msg) = payload.downcast_ref::<String>() {
            msg.to_string()
        } else {
            "Something went wrong".to_string()
        };

        let panic = Panic::new(message);

        let mut report: Report = if let Some(loc) = info.location() {
            PanicLocation::new(panic, loc).into()
        } else {
            panic.into()
        };

        if let Ok(Some(mut src_span)) = TestSourceSpan::from_backtrace(caller) {
            src_span.add_related(report);
            report = src_span.into();
        }

        eprintln!("{:?}", report);
    }));
}

#[track_caller]
fn init_miette() {
    INSTALLER.call_once(|| {
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .context_lines(3)
                    .tab_width(4)
                    .width(200)
                    .with_cause_chain()
                    .graphical_theme(GraphicalTheme {
                        characters: ThemeCharacters::unicode(),
                        styles: ThemeStyles {
                            highlights: vec![Style::new().red().bold()],
                            ..ThemeStyles::ansi()
                        },
                    })
                    .build(),
            )
        }))
        .expect("Error installing miette handler")
    });

    set_panic_hook();
}

fn main() {
    // init_miette();

    parser_v2::test().unwrap();
}
