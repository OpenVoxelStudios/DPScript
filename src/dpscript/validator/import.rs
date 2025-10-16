use crate::{
    common::traits::HasSpan,
    dpscript::{
        ast::{ast::ExportType, import::ImportNode},
        validator::{
            Result, Validator,
            err::{Err, VErr},
        },
    },
};

impl Validator {
    pub fn validate_imports(&mut self) -> Result<()> {
        let mut out = Vec::new();

        for mut node in self.ast.scope.imports.clone() {
            self.validate_import(&mut node)?;
            out.push(node);
        }

        self.ast.scope.imports = out;

        Ok(())
    }

    pub fn validate_import(&mut self, node: &mut ImportNode) -> Result<()> {
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

                    match value.clone() {
                        ExportType::Constant(node) => {
                            self.scope_mut()?.constants.insert(node.name.clone(), node);
                        }

                        ExportType::Objective(node) => {
                            self.scope_mut()?.objectives.insert(node.name.clone(), node);
                        }

                        ExportType::Function(node) => {
                            if let Some(recv) = &node.receiver {
                                self.scope_mut()?
                                    .instance_funcs
                                    .entry(recv.clone())
                                    .or_default()
                                    .insert(node.name.clone(), node);
                            } else {
                                self.scope_mut()?.functions.insert(node.name.clone(), node);
                            }
                        }

                        ExportType::Enum(node) => {
                            self.scope_mut()?.enums.insert(node.name.clone(), node);
                        }

                        ExportType::Field(node) => {
                            self.scope_mut()?
                                .fields
                                .entry(node.owner.clone())
                                .or_default()
                                .insert(node.name.clone(), node);
                        }
                    };
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

        Ok(())
    }
}
