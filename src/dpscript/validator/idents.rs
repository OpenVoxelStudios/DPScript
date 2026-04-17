use ast::data::Spanned;

use crate::dpscript::validator::{
    Result, Validator,
    err::{Err, Warn},
};

impl<'a> Validator<'a> {
    pub fn validate_ident(&mut self, id: Spanned<&'a str>) -> Result<()> {
        if id.0.to_lowercase() == "ib" {
            // :(
            self.warnings.push(Warn::IbPtsd { span: id.1.into() });
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
                span: id.1.into(),
                id: id.0.into(),
            });
        }

        Ok(())
    }

    pub fn validate_ident_literal(&mut self, id: Spanned<&'a str>) -> Result<()> {
        if self.scope()?.borrow().lookup(id.0).is_none() {
            self.errors.push(Err::UnresolvedRef {
                name: id.0.into(),
                span: id.1.into(),
            });
        }

        Ok(())
    }
}
