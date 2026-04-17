use std::rc::Rc;

use crate::dpscript::validator::{
    Result, Validator,
    err::{Err, VErr},
};
use ast::{data::HasSpan, import::ImportNode, scope::ExportType};

impl<'a> Validator<'a> {
    pub fn validate_imports(&mut self) -> Result<()> {
        let mut out = Vec::new();
        let imps = self.ast.borrow().scope.borrow().imports.clone();

        for mut node in imps {
            self.validate_import(&mut node)?;
            out.push(node);
        }

        self.ast.borrow().scope.borrow_mut().imports = out;

        Ok(())
    }

    pub fn validate_import(&mut self, node: &mut ImportNode<'a>) -> Result<()> {
        for (item, _span) in &node.imports {
            let mut item = item.clone();

            let name = item.pop().ok_or(VErr::EmptyImportPath {
                span: node.span().into(),
            })?;

            if self.imports.contains_key(&name) {
                self.errors.push(Err::DuplicateImport {
                    span: node.span().into(),
                    name: name.into(),
                });

                continue;
            }

            let module = item.join("::");

            if let Some(it) = self.modules.get(module.as_str()) {
                if let Some(values) = it.borrow().scope.borrow().exports.get(&name) {
                    for value in values.clone() {
                        self.imports.insert(name, value.clone());

                        match value {
                            ExportType::Constant(node) => {
                                self.scope()?
                                    .borrow_mut()
                                    .constants
                                    .insert(node.name.0, Rc::new(node));
                            }

                            ExportType::Objective(node) => {
                                self.scope()?
                                    .borrow_mut()
                                    .objectives
                                    .insert(node.name.0, Rc::new(node));
                            }

                            ExportType::Function(node) => {
                                if let Some(recv) = &node.receiver {
                                    self.scope()?
                                        .borrow_mut()
                                        .instance_funcs
                                        .entry(recv.0)
                                        .or_default()
                                        .insert(node.name.0, Rc::new(node));
                                } else {
                                    self.scope()?
                                        .borrow_mut()
                                        .functions
                                        .insert(node.name.0, Rc::new(node));
                                }
                            }

                            ExportType::Enum(node) => {
                                self.scope()?.borrow_mut().enums.insert(node.name.0, node);
                            }

                            ExportType::Field(node) => {
                                self.scope()?
                                    .borrow_mut()
                                    .fields
                                    .entry(node.owner.0)
                                    .or_default()
                                    .insert(node.name.0, Rc::new(node));
                            }
                        };
                    }
                } else {
                    self.errors.push(Err::UnresolvedImport {
                        span: node.span().into(),
                        path: name.into(),
                        module,
                    });
                }
            } else {
                self.errors.push(Err::ModuleNotFound {
                    module: module,
                    span: node.span().into(),
                });
            }
        }

        Ok(())
    }
}
