use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../dpscript.pest"]
pub struct PestParser;
