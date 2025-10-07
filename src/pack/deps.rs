use super::PackToml;
use crate::{Result, error::DependencyError};
use miette::{NamedSource, SourceOffset, SourceSpan};
use std::{collections::HashMap, fs, path::PathBuf};
use walkdir::WalkDir;

/// return: Vec<(namespace, keep (is_not_dependency), Vec<file>)>
pub fn get_source_files(
    dir: &PathBuf,
    pack: &PackToml,
) -> Result<Vec<(String, bool, Vec<String>)>> {
    let mut files = Vec::new();

    files.push((pack.pack.name.clone(), true, get_pack_source_files(dir)));

    files.extend(
        resolve_deps(pack)?
            .iter()
            .map(|(v, ns)| (ns.clone(), false, get_pack_source_files(v)))
            .collect::<Vec<_>>(),
    );

    Ok(files)
}

fn get_pack_source_files(dir: &PathBuf) -> Vec<String> {
    let root = dir.join("src");
    let mut files = Vec::new();

    let walk = WalkDir::new(&root)
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|v| v.file_name().to_str().unwrap().ends_with(".dps"))
        .collect::<Vec<_>>();

    for entry in walk {
        let path = entry.path();

        files.push(path.to_str().unwrap().into());
    }

    files
}

fn resolve_deps(proj: &PackToml) -> Result<HashMap<PathBuf, String>> {
    let mut dep_dirs = HashMap::new();

    for (item, path) in &proj.dependencies {
        let path = PathBuf::from(path);
        let file = path.join("pack.toml");
        let data = fs::read_to_string(&file)?;
        let toml = toml::from_str::<PackToml>(&data)?;

        if toml.pack.name != item.clone() {
            let src = NamedSource::new(file.to_str().unwrap(), data.clone());

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

        if !dep_dirs.contains_key(&path) {
            dep_dirs.insert(path, toml.pack.name.clone());
        }

        for (item, ns) in resolve_deps(&toml)? {
            if !dep_dirs.contains_key(&item) {
                dep_dirs.insert(item, ns);
            }
        }
    }

    Ok(dep_dirs)
}
