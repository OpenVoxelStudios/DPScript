use crate::{
    common::traits::HasSpan,
    dpscript::validator::{
        Result, Validator,
        err::{Err, VErr},
    },
};

impl Validator {
    pub fn validate_imports(&mut self) -> Result<()> {
        for node in &self.ast.imports {
            for item in &node.imports {
                let mut item = item.clone();

                let name = item
                    .pop()
                    .ok_or(VErr::EmptyImportPath { span: node.span() })?;

                if self.imports.contains_key(&name) {
                    self.errors.push(Err::DuplicateImport {
                        span: node.span(),
                        name,
                    });

                    continue;
                }

                let module = item.join("::");

                if let Some(it) = self.modules.get(&module) {
                    if let Some(value) = it.exports.get(&name) {
                        self.imports.insert(name, value.clone());
                    } else {
                        self.errors.push(Err::UnresolvedImport {
                            span: node.span(),
                            path: name,
                            module,
                        });
                    }
                } else {
                    self.errors.push(Err::ModuleNotFound {
                        module: module,
                        span: node.span(),
                    });
                }
            }
        }

        Ok(())
    }
}
