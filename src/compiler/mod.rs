use crate::{
    Result,
    dpscript::{ast::ast::AST, lexer::FullLexer, tokenizer::Tokenizer, validator::Validator},
    pack::{PackToml, get_pack_source_files, resolve_pack_deps},
};
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compiler {
    pub base: PathBuf,
    pub config: PackToml,
    pub config_path: PathBuf,
    pub out_dir: PathBuf,
    pub dump_tokens: bool,
    pub dump_ast: bool,
    pub dump_ir: bool,
    pub allow_dead_code: bool,
}

impl Compiler {
    pub fn new(
        config_path: PathBuf,
        out_dir: Option<PathBuf>,
        dump_tokens: bool,
        dump_ast: bool,
        dump_ir: bool,
        allow_dead_code: bool,
    ) -> Result<Self> {
        let base = config_path.canonicalize()?.parent().unwrap().to_path_buf();
        let config = fs::read_to_string(&config_path)?;
        let config = toml::from_str::<PackToml>(&config)?;

        let out_dir = out_dir
            .clone()
            .unwrap_or(PathBuf::from(config.build.output.clone()));

        Ok(Self {
            config_path,
            base,
            config,
            dump_ast,
            dump_ir,
            dump_tokens,
            out_dir,
            allow_dead_code,
        })
    }

    pub fn compile_project(&self) -> Result<()> {
        let dump_dir = self.out_dir.join(".dpscript");
        let pkgs = resolve_pack_deps(&self.base)?;

        if !dump_dir.exists() {
            fs::create_dir_all(&dump_dir)?;
        }

        let style = ProgressStyle::with_template(
            "[{bar:40.cyan/blue}] {pos:.blue} of {len:.blue}",
        )
        .unwrap()
        .progress_chars("=> ");

        let mut asts: Vec<AST> = Vec::new();
        let pb = MultiProgress::new();
        let master = pb.add(ProgressBar::new(pkgs.len() as u64).with_style(style.clone()));

        for pkg in pkgs {
            master.inc(1);

            master.println(format!(
                "   {} {} {}{}",
                "Parsing".green().bold(),
                pkg.pack.pack.name.cyan().bold(),
                "v".magenta().bold(),
                pkg.pack.pack.version.magenta().bold()
            ));

            let files = get_pack_source_files(&pkg.src_path);
            let fpb = pb.add(ProgressBar::new(files.len() as u64).with_style(style.clone()));

            for file in files {
                let path = file.strip_prefix(&pkg.src_path).unwrap();
                let path = format!("{}", path.display());

                fpb.inc(1);

                fpb.println(format!(
                    "      + {} {}",
                    "Compiling".purple().bold(),
                    path.blue().bold()
                ));

                let path = path.trim_end_matches(".dps");
                let module = path.replace("\\", "::").replace("/", "::");
                let module = format!("{}::{}", pkg.pack.pack.name, module);

                asts.push(self.create_ast(
                    module,
                    &pkg.pack.pack.name,
                    &file,
                    pkg.keep || self.allow_dead_code,
                )?);
            }

            fpb.finish_and_clear();
        }

        master.finish();

        if asts.is_empty() {
            warn!("No source files found!");

            return Ok(());
        }

        let modules = Arc::new(
            asts.clone()
                .into_iter()
                .map(|it| (it.module.clone(), it))
                .collect::<HashMap<_, _>>(),
        );

        let mut warnings = 0;
        let pb = ProgressBar::new(asts.len() as u64).with_style(style.clone());

        for ast in asts.clone() {
            pb.inc(1);

            pb.println(format!(
                "   {} {}",
                "Analyzing".green().bold(),
                ast.module.cyan().bold(),
            ));

            let errs = Validator::new(ast, Arc::clone(&modules)).run()?;

            if !errs.errors.is_empty() {
                pb.finish();

                return Err(errs.into());
            }

            if !errs.warnings.is_empty() {
                for warn in errs.warnings {
                    pb.println(format!(
                        "{:?}",
                        miette::Report::new(warn).with_source_code(errs.code.clone())
                    ));

                    warnings += 1;
                }
            }
        }

        pb.finish();

        warn!("Generated {warnings} warnings.");

        // if self.dump_ast {
        // let dump_file = dump_dir.join("merged.nodes.ron");

        // fs::write(
        //     dump_file,
        //     ron::ser::to_string_pretty(&full.nodes, PrettyConfig::new())?,
        // )?;

        // let dump_file = dump_dir.join("merged.nodes.dpir");

        // fs::write(
        //     dump_file,
        //     full.nodes
        //         .iter()
        //         .map(|it| format!("{it}"))
        //         .collect::<Vec<_>>()
        //         .join("\n\n"),
        // )?;

        // let dump_file = dump_dir.join("merged.ast.ron");

        // fs::write(
        //     dump_file,
        //     ron::ser::to_string_pretty(&full, PrettyConfig::new())?,
        // )?;

        // let merged_dir = dump_dir.join("merged");

        // if !merged_dir.exists() {
        //     fs::create_dir_all(&merged_dir)?;
        // }

        // dump_ast_part!(ast.top_level => merged_dir);
        // dump_ast_part!(ast.imports => merged_dir);
        // dump_ast_part!(ast.funcs => merged_dir);
        // dump_ast_part!(ast.vars => merged_dir);
        // dump_ast_part!(ast.blocks => merged_dir);
        // dump_ast_part!(ast.enums => merged_dir);
        // dump_ast_part!(ast.objectives => merged_dir);
        // dump_ast_part!(ast.modules => merged_dir);
        // dump_ast_part!(ast.exports => merged_dir);

        // if let Ok(it) = &ast.export_nodes() {
        //     let path = merged_dir.join("export_nodes.ron");

        //     fs::write(path, ron::ser::to_string_pretty(it, PrettyConfig::new())?)?;
        // }
        // }

        Ok(())
    }

    fn create_ast(
        &self,
        module: String,
        namespace: impl AsRef<str>,
        file: &PathBuf,
        keep: bool,
    ) -> Result<AST> {
        let file_name = file.to_str().unwrap();
        let data = fs::read_to_string(&file)?;
        let tokens = Tokenizer::new(&file_name, data.clone()).run()?.tokens();
        let dump_dir = self.out_dir.join(".dpscript");
        let parent = file.parent().unwrap();
        let mut parts = Vec::new();
        let mut found = false;

        parts.push(namespace.as_ref().into());

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

        let ast = FullLexer::new(
            module,
            namespace.as_ref().into(),
            file_name.into(),
            data,
            keep,
            tokens,
        )
        .run()?;

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
