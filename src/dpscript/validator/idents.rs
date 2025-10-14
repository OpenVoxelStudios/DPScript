use flexstr::SharedStr;

use crate::{
    dpscript::validator::{
        Result, Validator,
        err::{Err, Warn},
    },
    util::Spanned,
};

impl Validator {
    pub fn validate_ident(&mut self, id: &Spanned<SharedStr>) -> Result<()> {
        if id.0.to_lowercase() == "ib" {
            // :(
            self.warnings.push(Warn::IbPtsd { span: id.1 });
        }

        // A regex was the original impl, but this is pretty slow compared to other things
        // Looking at the flamegraph, regex_automata took about half of the processing
        // time, and removing only this cut it down to about a fifth or so

        let valid = id.0.chars().all(|it| it.is_alphanumeric() || it == '_')
            && id
                .0
                .chars()
                .next()
                .is_some_and(|it| it.is_alphabetic() || it == '_');

        if !valid {
            self.errors.push(Err::InvalidIdent {
                span: id.1,
                id: id.0.clone(),
            });
        }

        Ok(())
    }
}
