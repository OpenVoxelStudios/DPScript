mod node;
mod pack;
mod tag;

pub use pack::*;
pub use tag::*;

use super::IRAst;
use crate::{PackToml, Result, UnsourcedCompilerError, get_versions};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct IRCompiler {
    pub ast: IRAst,
}

impl IRCompiler {
    pub fn new(ast: IRAst) -> Self {
        Self { ast }
    }

    pub fn run(mut self, out_dir: impl Into<PathBuf>, config: &PackToml) -> Result<Self> {
        let out_dir = out_dir.into();

        self.ast.indexed = false;
        self.ast.index();

        let cx = self.ast.create_checker_context();

        for node in &self.ast.nodes {
            node.compile(&cx, &out_dir)?;
        }

        let path = out_dir.join("pack.mcmeta");
        let vers = get_versions();

        let Some(pack_format) = vers.get(&config.version.minecraft.as_str()) else {
            return Err(UnsourcedCompilerError {
                err: format!(
                    "Cannot find a pack_format for Minecraft version {}!",
                    config.version.minecraft
                ),
            }
            .into());
        };

        let info = PackInfo {
            pack: PackMeta {
                pack_format: *pack_format,
                description: config.pack.description.clone().unwrap_or_default(),
            },
        };

        if !out_dir.exists() {
            fs::create_dir_all(out_dir)?;
        }

        fs::write(path, serde_json::to_string_pretty(&info)?)?;

        Ok(self)
    }
}
