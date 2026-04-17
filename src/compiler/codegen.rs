use crate::{
    Result,
    compiler::{Compiler, STYLE},
    dpscript::{compiler::CodeGenerator, validator::ValidationResult},
};
use ast::ast::AST;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

impl Compiler {
    pub fn generate<'a>(
        &'a self,
        validated: Vec<ValidationResult<'a>>,
        modules: &Arc<HashMap<&'a str, Rc<RefCell<AST<'a>>>>>,
    ) -> Result<()> {
        let pb = if self.quiet {
            None
        } else {
            Some(MultiProgress::new())
        };

        let master = pb
            .as_ref()
            .map(|pb| pb.add(ProgressBar::new(validated.len() as u64).with_style(STYLE.clone())));

        for item in validated {
            if let Some(master) = &master {
                master.inc(1);

                master.println(format!(
                    "   {} {}",
                    "Compiling".green().bold(),
                    item.ast.borrow().module.cyan().bold(),
                ));
            }

            let code = item.ast.borrow().code.clone().into();

            let cg = CodeGenerator::new(
                code,
                self.out_dir.clone(),
                item.ast,
                item.imports,
                Arc::clone(modules),
            );

            cg.run()?;
        }

        if let Some(master) = master {
            master.finish_and_clear();
        }

        Ok(())
    }
}
