use crate::{
    Result,
    pack::{PackToml, resolve_pack_deps},
};
use indicatif::ProgressStyle;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

pub mod analysis;
pub mod codegen;
pub mod validation;

pub const STYLE: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:.blue} of {len:.blue}")
        .unwrap()
        .progress_chars("=> ")
});

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
    pub detailed: bool,
    pub quiet: bool,
}

impl Compiler {
    pub fn new(
        config_path: PathBuf,
        out_dir: Option<PathBuf>,
        dump_tokens: bool,
        dump_ast: bool,
        dump_ir: bool,
        allow_dead_code: bool,
        detailed: bool,
        quiet: bool,
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
            detailed,
            quiet,
        })
    }

    pub fn compile_project(&self) -> Result<()> {
        let dump_dir = self.out_dir.join(".dpscript");
        let pkgs = resolve_pack_deps(&self.base)?;

        if !dump_dir.exists() {
            fs::create_dir_all(&dump_dir)?;
        }

        let (asts, modules) = self.analyze(&pkgs)?;

        if asts.is_empty() {
            if !self.quiet {
                warn!("No source files found!");
            }

            return Ok(());
        }

        let validated = self.validate(&asts, &modules)?;

        self.generate(validated, &modules)?;

        Ok(())
    }
}
