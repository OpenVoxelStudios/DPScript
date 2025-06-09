use crate::{
    AST, IRAst, IRCompiler, IRFinalizer, IRLexer, IRTokenizer, IRValidator, Lexer, Lowerer,
    PackToml, Result, Tokenizer, Validator, dump_ast_part, get_source_files,
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
    pub build_ir: bool,
    pub dump_tokens: bool,
    pub dump_ast: bool,
    pub dump_ir: bool,
}

impl Compiler {
    pub fn new(
        config_path: PathBuf,
        out_dir: Option<PathBuf>,
        build_ir: bool,
        dump_tokens: bool,
        dump_ast: bool,
        dump_ir: bool,
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
            build_ir,
            config,
            dump_ast,
            dump_ir,
            dump_tokens,
            out_dir,
        })
    }

    pub fn compile_project(&self) -> Result<()> {
        let dump_dir = self.out_dir.join(".dpscript");
        let source_files = get_source_files(&self.base, &self.config, self.build_ir)?;

        if !dump_dir.exists() {
            fs::create_dir_all(&dump_dir)?;
        }

        if self.build_ir {
            let style =
                ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:.blue} of {len:.blue}")
                    .unwrap()
                    .progress_chars("## ");

            let mut asts = Vec::new();

            for path in source_files.iter().progress_with_style(style) {
                asts.push(self.create_ir_ast(&PathBuf::from(path))?);
            }

            if asts.is_empty() {
                warn!("No source files found!");

                return Ok(());
            }

            let mut ast = asts.remove(0);

            for item in asts {
                ast.merge(item);
            }

            if self.dump_ast {
                let dump_file = dump_dir.join("ir_ast_merged.ron");

                fs::write(
                    dump_file,
                    ron::ser::to_string_pretty(&ast, PrettyConfig::new())?,
                )?;

                let dump_file = dump_dir.join("ir_ast_merged.back_conv.dpir");

                fs::write(dump_file, ast.serialize_nodes())?;
            }

            let ast = IRValidator::new(ast).run()?.ast.clone();

            if self.dump_ast {
                let dump_file = dump_dir.join("ir_ast_merged_validated.ron");

                fs::write(
                    dump_file,
                    ron::ser::to_string_pretty(&ast, PrettyConfig::new())?,
                )?;
            }
        } else {
            let style =
                ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:.blue} of {len:.blue}")
                    .unwrap()
                    .progress_chars("## ");

            let mut asts = Vec::new();

            for path in source_files.iter().progress_with_style(style) {
                asts.push(self.create_ast(&PathBuf::from(path))?);
            }

            if asts.is_empty() {
                warn!("No source files found!");

                return Ok(());
            }

            let mut ast = asts.remove(0);

            for item in asts {
                ast.merge(item);
            }

            ast.index()?;

            if self.dump_ast {
                let dump_file = dump_dir.join("ast_merged.ron");

                fs::write(
                    dump_file,
                    ron::ser::to_string_pretty(&ast, PrettyConfig::new())?,
                )?;

                let merged_dir = dump_dir.join("merged");

                if !merged_dir.exists() {
                    fs::create_dir_all(&merged_dir)?;
                }

                dump_ast_part!(ast.top_level => merged_dir);
                dump_ast_part!(ast.imports => merged_dir);
                dump_ast_part!(ast.funcs => merged_dir);
                dump_ast_part!(ast.vars => merged_dir);
                dump_ast_part!(ast.blocks => merged_dir);
                dump_ast_part!(ast.enums => merged_dir);
                dump_ast_part!(ast.objectives => merged_dir);
                dump_ast_part!(ast.modules => merged_dir);
                dump_ast_part!(ast.exports => merged_dir);

                if let Ok(it) = &ast.export_nodes() {
                    let path = merged_dir.join("export_nodes.ron");

                    fs::write(path, ron::ser::to_string_pretty(it, PrettyConfig::new())?)?;
                }
            }

            let ast = Validator::new(ast).run()?.ast.clone();

            if self.dump_ast {
                let dump_file = dump_dir.join("ast_merged_validated.ron");

                fs::write(
                    dump_file,
                    ron::ser::to_string_pretty(&ast, PrettyConfig::new())?,
                )?;
            }

            let lowered = Lowerer::new(&self.config.pack.name, ast).run()?;

            if self.dump_ir {
                let dump_file = dump_dir.join("code_merged.dpir.ron");

                fs::write(
                    dump_file,
                    &ron::ser::to_string_pretty(
                        &lowered.lowered.clone().unwrap(),
                        PrettyConfig::new(),
                    )?,
                )?;

                let dump_file = dump_dir.join("code_merged.dpir");

                fs::write(dump_file, &lowered.get_code()?)?;
            }

            let validator = IRValidator::new(lowered.lowered.unwrap()).run()?;

            if self.dump_ir {
                let dump_file = dump_dir.join("code_merged_firstpass.dpir.ron");

                fs::write(
                    dump_file,
                    &ron::ser::to_string_pretty(&validator.ast, PrettyConfig::new())?,
                )?;

                let dump_file = dump_dir.join("code_merged_firstpass.dpir");

                fs::write(dump_file, &validator.get_code())?;
            }

            let finalizer = IRFinalizer::new(validator.ast).run()?;

            if self.dump_ir {
                let dump_file = dump_dir.join("code_merged_finalized.dpir.ron");

                fs::write(
                    dump_file,
                    &ron::ser::to_string_pretty(&finalizer.ast, PrettyConfig::new())?,
                )?;

                let dump_file = dump_dir.join("code_merged_finalized.dpir");

                fs::write(dump_file, &finalizer.get_code())?;
            }

            IRCompiler::new(finalizer.ast).run(&self.out_dir, &self.config)?;
        }

        Ok(())
    }

    fn create_ast(&self, file: &PathBuf) -> Result<AST> {
        let file_name = file.to_str().unwrap();
        let data = fs::read_to_string(&file)?;
        let tokens = Tokenizer::new(&file_name, data.clone()).run()?.tokens();
        let dump_dir = self.out_dir.join(".dpscript");

        if !dump_dir.exists() {
            fs::create_dir_all(&dump_dir)?;
        }

        if self.dump_tokens {
            let tokens_dir = dump_dir.join("tokens");

            if !tokens_dir.exists() {
                fs::create_dir_all(&tokens_dir)?;
            }

            let dump_file =
                tokens_dir.join(file.with_extension("dps.tokens.ron").file_name().unwrap());

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&tokens, PrettyConfig::new())?,
            )?;
        }

        let ast = Lexer::new(&file_name, data, tokens).run()?.ast();

        if self.dump_ast {
            let ast_dir = dump_dir.join("ast");

            if !ast_dir.exists() {
                fs::create_dir_all(&ast_dir)?;
            }

            let dump_file = ast_dir.join(file.with_extension("dps.ast.ron").file_name().unwrap());

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&ast, PrettyConfig::new())?,
            )?;
        }

        Ok(ast)
    }

    fn create_ir_ast(&self, file: &PathBuf) -> Result<IRAst> {
        let data = fs::read_to_string(&file)?;
        let tokens = IRTokenizer::new(data.clone()).tokenize()?.tokens();
        let dump_dir = self.out_dir.join(".dpscript");

        if !dump_dir.exists() {
            fs::create_dir_all(&dump_dir)?;
        }

        if self.dump_tokens {
            let tokens_dir = dump_dir.join("tokens");

            if !tokens_dir.exists() {
                fs::create_dir_all(&tokens_dir)?;
            }

            let dump_file =
                tokens_dir.join(file.with_extension("dpir.tokens.ron").file_name().unwrap());

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&tokens, PrettyConfig::new())?,
            )?;
        }

        let ast = IRLexer::new(data, tokens).run()?.ast();

        if self.dump_ast {
            let ast_dir = dump_dir.join("ast");

            if !ast_dir.exists() {
                fs::create_dir_all(&ast_dir)?;
            }

            let dump_file = ast_dir.join(file.with_extension("dps.ast.ron").file_name().unwrap());

            fs::write(
                dump_file,
                ron::ser::to_string_pretty(&ast, PrettyConfig::new())?,
            )?;
        }

        Ok(ast)
    }
}
