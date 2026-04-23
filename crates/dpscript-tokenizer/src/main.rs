use std::error::Error;

use dpscript_tokenizer::tokenize;

pub fn main() -> Result<(), Box<dyn Error>> {
    let data = include_str!("../../../std/src/types/types.dps");
    let tokens = tokenize(data)?;

    println!("{tokens:#?}");

    Ok(())
}
