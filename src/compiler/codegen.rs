use std::{collections::HashMap, sync::Arc};

use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar};

use crate::{
    Result,
    compiler::{Compiler, STYLE},
    dpscript::{ast::ast::AST, compiler::CodeGenerator, validator::ValidationResult},
};

impl Compiler {
    pub fn generate(
        &self,
        validated: Vec<ValidationResult>,
        modules: &Arc<HashMap<String, AST>>,
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
                    item.ast.module.cyan().bold(),
                ));
            }

            let cg = CodeGenerator::new(
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
