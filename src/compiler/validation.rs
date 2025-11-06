use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar};

use crate::{
    Result,
    compiler::{Compiler, STYLE},
    dpscript::{
        ast::ast::AST,
        validator::{ValidationResult, Validator},
    },
    error::CompleteValidationErrors,
};

impl Compiler {
    pub fn validate(
        &self,
        asts: &Vec<AST>,
        mut modules: Arc<HashMap<String, AST>>,
    ) -> Result<(Vec<ValidationResult>, Arc<HashMap<String, AST>>)> {
        let mut warnings = 0;
        let mut pretty = BTreeMap::new();

        for ast in asts {
            pretty.entry(&ast.namespace).or_insert(Vec::new()).push(ast);
        }

        let mut analyzed = Vec::new();
        let mut errors = Vec::new();

        let pb = if self.quiet {
            None
        } else {
            Some(MultiProgress::new())
        };

        let master = pb
            .as_ref()
            .map(|pb| pb.add(ProgressBar::new(pretty.len() as u64).with_style(STYLE.clone())));

        for (ns, asts) in pretty {
            if let Some(master) = &master {
                master.inc(1);

                master.println(format!(
                    "   {} {}",
                    "Analyzing".green().bold(),
                    ns.cyan().bold(),
                ));
            }

            let mpb = pb
                .as_ref()
                .map(|pb| pb.add(ProgressBar::new(asts.len() as u64).with_style(STYLE.clone())));

            for ast in asts {
                if let Some(mpb) = &mpb {
                    mpb.inc(1);

                    if self.detailed {
                        mpb.println(format!(
                            "      + {} {}",
                            "Analyzing".purple().bold(),
                            ast.module.blue().bold()
                        ));
                    }
                }

                let result = Validator::new(ast.clone(), Arc::clone(&modules)).run()?;

                if !result.errors.errors.is_empty() {
                    errors.push(result.errors.into());
                    continue;
                }

                if !result.errors.warnings.is_empty() {
                    for warn in &result.errors.warnings {
                        let msg = format!(
                            "{:?}",
                            miette::Report::new(warn.clone())
                                .with_source_code(result.errors.code.clone())
                        );

                        if let Some(mpb) = &mpb {
                            mpb.println(msg);
                        } else {
                            println!("{}", msg);
                        }

                        warnings += 1;
                    }
                }

                let mut modules_inner = Arc::into_inner(modules).unwrap();

                modules_inner.insert(result.ast.module.clone(), result.ast.clone());
                modules = Arc::new(modules_inner);
                analyzed.push(result);
            }

            if let Some(mpb) = mpb {
                mpb.finish_and_clear();
            }
        }

        if let Some(master) = master {
            master.finish_and_clear();
        }

        if !errors.is_empty() {
            return Err(CompleteValidationErrors { errors }.into());
        }

        warn!("Generated {warnings} warnings.");

        Ok((analyzed, modules))
    }
}
