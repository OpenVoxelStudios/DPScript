use crate::{
    common::traits::HasSpan,
    dpscript::validator::{
        Result, Validator,
        err::{Err, VErr},
    },
};

impl Validator {
    pub fn validate_imports(&mut self) -> Result<()> {
        for node in &self.ast.scope.imports {
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

                let module = item.join("::").into();

                if let Some(it) = self.modules.get(&module) {
                    if let Some(value) = it.scope.exports.get(&name) {
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
