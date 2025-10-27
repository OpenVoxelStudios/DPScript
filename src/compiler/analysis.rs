use crate::{
    Result,
    compiler::{Compiler, STYLE},
    dpscript::{ast::ast::AST, lexer::FullLexer, tokenizer::Tokenizer},
    pack::{PackageInfo, get_pack_source_files},
};
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar};
use ron::ser::PrettyConfig;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

impl Compiler {
    pub fn analyze(
        &self,
        pkgs: &Vec<PackageInfo>,
    ) -> Result<(Vec<AST>, Arc<HashMap<String, AST>>)> {
        let mut asts: Vec<AST> = Vec::new();

        let pb = if self.quiet {
            None
        } else {
            Some(MultiProgress::new())
        };

        let master = pb
            .as_ref()
            .map(|pb| pb.add(ProgressBar::new(pkgs.len() as u64).with_style(STYLE.clone())));

        for pkg in pkgs {
            if let Some(master) = &master {
                master.inc(1);

                master.println(format!(
                    "   {} {} {}{}",
                    "Parsing".green().bold(),
                    pkg.pack.pack.name.cyan().bold(),
                    "v".magenta().bold(),
                    pkg.pack.pack.version.magenta().bold()
                ));
            }

            let files = get_pack_source_files(&pkg.src_path);
            let ns: String = pkg.pack.pack.name.clone().into();

            let fpb = pb
                .as_ref()
                .map(|pb| pb.add(ProgressBar::new(files.len() as u64).with_style(STYLE.clone())));

            for file in files {
                let path = file.strip_prefix(&pkg.src_path).unwrap();
                let path = format!("{}", path.display());

                if let Some(fpb) = &fpb {
                    fpb.inc(1);

                    if self.detailed {
                        fpb.println(format!(
                            "      + {} {}",
                            "Parsing".purple().bold(),
                            path.blue().bold()
                        ));
                    }
                }

                let path = path.trim_end_matches(".dps");
                let module = path.replace("\\", "::").replace("/", "::");
                let module = format!("{}::{}", pkg.pack.pack.name, module);

                asts.push(self.create_ast(
                    module.into(),
                    ns.clone(),
                    &file,
                    pkg.keep || self.allow_dead_code,
                )?);
            }

            if let Some(fpb) = fpb {
                fpb.finish_and_clear();
            }
        }

        if let Some(master) = master {
            master.finish_and_clear();
        }

        let modules = Arc::new(
            asts.clone()
                .into_iter()
                .map(|it| (it.module.clone(), it))
                .collect::<HashMap<_, _>>(),
        );

        Ok((asts, modules))
    }

    fn create_ast(
        &self,
        module: String,
        namespace: String,
        file: &PathBuf,
        keep: bool,
    ) -> Result<AST> {
        let file_name = file.to_str().unwrap();
        let data: String = fs::read_to_string(&file)?.into();
        let tokens = Tokenizer::new(&file_name, data.clone()).run()?.tokens();
        let dump_dir = self.out_dir.join(".dpscript");
        let parent = file.parent().unwrap();
        let mut parts = Vec::new();
        let mut found = false;

        parts.push(namespace.clone());

        for part in parent {
            if part == "src" && !found {
                found = true;
                continue;
            }

            if found {
                parts.push(part.to_string_lossy().to_string());
            }
        }

        let dump_dir = dump_dir.join(parts.join("/"));

        if !dump_dir.exists() {
            fs::create_dir_all(&dump_dir)?;
        }

        if self.dump_tokens {
            let dump_file =
                dump_dir.join(file.with_extension("dps.tokens.ron").file_name().unwrap());

            let dump_str_file = dump_dir.join(
                file.with_extension("dps.str_tokens.dps")
                    .file_name()
                    .unwrap(),
            );

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&tokens, PrettyConfig::new())?,
            )?;

            fs::write(
                dump_str_file,
                tokens
                    .iter()
                    .map(|it| format!("{}", it.0))
                    .collect::<Vec<_>>()
                    .join(" "),
            )?;
        }

        let ast = FullLexer::new(module, namespace, file_name.into(), data, keep, tokens).run()?;

        if self.dump_ast {
            let dump_file =
                dump_dir.join(file.with_extension("dps.nodes.ron").file_name().unwrap());

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&ast.nodes, PrettyConfig::new())?,
            )?;

            let dump_file_2 =
                dump_dir.join(file.with_extension("dps.nodes.dpir").file_name().unwrap());

            fs::write(
                dump_file_2,
                ast.nodes
                    .iter()
                    .map(|it| format!("{it}"))
                    .collect::<Vec<_>>()
                    .join("\n\n"),
            )?;

            let dump_file_3 =
                dump_dir.join(file.with_extension("dps.ast.ron").file_name().unwrap());

            fs::write(
                dump_file_3,
                ron::ser::to_string_pretty(&ast, PrettyConfig::new())?,
            )?;
        }

        Ok(ast)
    }
}
