use crate::{
    Result,
    dpscript::{ast::ast::AST, lexer::FullLexer, tokenizer::Tokenizer},
    pack::{PackToml, get_source_files},
};
use indicatif::{ProgressIterator, ProgressStyle};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

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
        let source_files = get_source_files(&self.base, &self.config)?;

        if !dump_dir.exists() {
            fs::create_dir_all(&dump_dir)?;
        }

        let style = ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:.blue} of {len:.blue}")
            .unwrap()
            .progress_chars("## ");

        let mut asts = Vec::new();

        for (ns, keep, path) in source_files
            .into_iter()
            .flat_map(|(ns, keep, pat)| pat.into_iter().map(move |it| (ns.clone(), keep, it)))
            .collect::<Vec<_>>()
            .into_iter()
            .progress_with_style(style)
        {
            asts.push(self.create_ast(ns, &PathBuf::from(path), keep || self.allow_dead_code)?);
        }

        if asts.is_empty() {
            warn!("No source files found!");

            return Ok(());
        }

        let mut full = AST::new(Vec::new());

        for ast in asts {
            full = full.merge(ast);
        }

        if self.dump_ast {
            let dump_file = dump_dir.join("merged.nodes.ron");

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&full.nodes, PrettyConfig::new())?,
            )?;

            let dump_file = dump_dir.join("merged.nodes.dpir");

            fs::write(
                dump_file,
                full.nodes
                    .iter()
                    .map(|it| format!("{it}"))
                    .collect::<Vec<_>>()
                    .join("\n\n"),
            )?;

            let dump_file = dump_dir.join("merged.ast.ron");

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&full, PrettyConfig::new())?,
            )?;

            let merged_dir = dump_dir.join("merged");

            if !merged_dir.exists() {
                fs::create_dir_all(&merged_dir)?;
            }

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
        }

        Ok(())
    }

    fn create_ast(&self, namespace: impl AsRef<str>, file: &PathBuf, keep: bool) -> Result<AST> {
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
