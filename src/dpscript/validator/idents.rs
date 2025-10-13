use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    dpscript::validator::{
        Result, Validator,
        err::{Err, Warn},
    },
    util::Spanned,
};

pub const IDENT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^[A-Za-z_][A-Za-z0-9_]*$").unwrap());

impl Validator {
    pub fn validate_ident(&mut self, id: &Spanned<String>) -> Result<()> {
        if id.0.to_lowercase() == "ib" {
            // :(
            self.warnings.push(Warn::IbPtsd { span: id.1 });
        }

        if !IDENT_REGEX.is_match(&id.0) {
            self.errors.push(Err::InvalidIdent {
                span: id.1,
                id: id.0.clone(),
            });
        }

        Ok(())
    }
}
