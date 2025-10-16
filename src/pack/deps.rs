use super::PackToml;
use crate::{Result, error::DependencyError};
use miette::{NamedSource, SourceOffset, SourceSpan};
use std::{collections::HashMap, fs, path::PathBuf};
use walkdir::WalkDir;

pub fn get_pack_source_files(src_dir: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let walk = WalkDir::new(&src_dir)
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|v| v.file_name().to_str().unwrap().ends_with(".dps"))
        .collect::<Vec<_>>();

    for entry in walk {
        files.push(entry.path().to_path_buf());
    }

    files
}

pub fn resolve_pack_deps(root: &PathBuf) -> Result<Vec<PackageInfo>> {
    Ok(resolve_deps(root, true)?.into_values().collect())
}

fn resolve_deps(path: &PathBuf, root: bool) -> Result<HashMap<String, PackageInfo>> {
    let proj = toml::from_str::<PackToml>(&fs::read_to_string(path.join("pack.toml"))?)?;
    let mut dep_dirs = HashMap::new();

    dep_dirs.insert(
        proj.pack.name.clone(),
        PackageInfo {
            pack: proj.clone(),
            path: path.clone(),
            src_path: path.join("src"),
            keep: root,
        },
    );

    for (item, path) in &proj.dependencies {
        let path = PathBuf::from(path);
        let file = path.join("pack.toml");
        let data = fs::read_to_string(&file)?;
        let toml = toml::from_str::<PackToml>(&data)?;

        if toml.pack.name != item.clone() {
            let src = NamedSource::new(file.to_str().unwrap(), data.clone()).with_language("toml");

            return Err(DependencyError {
                src,
                at: SourceSpan::new(SourceOffset::from_location(data, 0, 0), 1),
                err: format!(
                    "Dependency named \"{}\" does not match required name \"{}\"!",
                    toml.pack.name, item
                ),
            }
            .into());
        }

        let path = path.canonicalize()?;

        dep_dirs.extend(resolve_deps(&path, false)?);
    }

    Ok(dep_dirs)
}

/// Package info.
#[derive(Debug, Clone, PartialEq)]
pub struct PackageInfo {
    /// The package's pack.toml info.
    pub pack: PackToml,

    /// The root path to the package.
    pub path: PathBuf,

    /// The sources path of the package.
    pub src_path: PathBuf,

    /// Whether to exclude this package's code from dead code elimination.
    /// This will be true if this is the root project we are compiling.
    pub keep: bool,
}
