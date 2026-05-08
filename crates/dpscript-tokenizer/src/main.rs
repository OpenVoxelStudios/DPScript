use dpscript_tokenizer::tokenize_file;
use miette::IntoDiagnostic;
use std::fs;

pub fn main() -> miette::Result<()> {
    let file = "std/src/std.dps";
    let content = fs::read_to_string(&file).into_diagnostic()?;
    let tokens = tokenize_file(&file, &content)?;

    println!(
        "{}",
        tokens
            .into_iter()
            .map(|it| format!("{}", it.0))
            .collect::<Vec<_>>()
            .join(" ")
    );

    Ok(())
}
