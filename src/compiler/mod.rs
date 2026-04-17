use crate::{
    Result,
    pack::{PackToml, PackageInfo, get_pack_source_files, resolve_pack_deps},
};
use indicatif::ProgressStyle;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::PathBuf};

pub mod analysis;
pub mod codegen;
pub mod validation;

pub const STYLE: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:.blue} of {len:.blue}")
        .unwrap()
        .progress_chars("=> ")
});

#[derive(Debug, Clone, Serialize)]
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

    /// A map of file names to their content.
    pub files: BTreeMap<PathBuf, String>,
    pub file_names: BTreeMap<PathBuf, String>,
    pub modules: BTreeMap<PathBuf, String>,
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

        let pkgs = resolve_pack_deps(&base)?;
        let files = collect_files(pkgs)?;
        let mut file_names = BTreeMap::new();

        for (k, _) in &files {
            let v = format!("{}", k.display());

            file_names.insert(k.clone(), v);
        }

        let mut modules = BTreeMap::new();

        for (k, (_, sp, name)) in &files {
            let path = k.strip_prefix(sp).unwrap();
            let path = format!("{}", path.display());
            let path = path.trim_end_matches(".dps");
            let module = path.replace("\\", "::").replace("/", "::");
            let module = format!("{}::{}", name, module);

            modules.insert(k.clone(), module);
        }

        let files = files.into_iter().map(|(a, b)| (a, b.0)).collect();

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
            files,
            file_names,
            modules,
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

        let (validated, modules) = self.validate(&asts, modules)?;

        self.generate(validated, &modules)?;

        Ok(())
    }
}

pub fn collect_files(
    pkgs: Vec<PackageInfo>,
) -> Result<BTreeMap<PathBuf, (String, PathBuf, String)>> {
    let mut content = BTreeMap::new();

    for pkg in pkgs {
        let files = get_pack_source_files(&pkg.src_path);

        for file in files {
            let data = fs::read_to_string(&file)?;

            content.insert(
                file,
                (data, pkg.src_path.clone(), pkg.pack.pack.name.clone()),
            );
        }
    }

    Ok(content)
}
