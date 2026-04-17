use crate::{
    Result,
    compiler::{Compiler, STYLE},
    error::Error,
    pack::{PackageInfo, get_pack_source_files},
};
use ast::ast::AST;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar};
use parser_v2::FileParser;
use ron::ser::PrettyConfig;
use std::{cell::RefCell, collections::HashMap, fs, path::PathBuf, rc::Rc, sync::Arc};

impl Compiler {
    pub fn analyze<'a>(
        &'a self,
        pkgs: &'a Vec<PackageInfo>,
    ) -> Result<(
        Vec<Rc<RefCell<AST<'a>>>>,
        Arc<HashMap<&'a str, Rc<RefCell<AST<'a>>>>>,
    )> {
        let mut asts = Vec::new();

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
            let ns = &pkg.pack.pack.name;

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

                let ast = self.create_ast(ns, file, pkg.keep || self.allow_dead_code)?;

                asts.push((ast.module, Rc::new(RefCell::new(ast))));
            }

            if let Some(fpb) = fpb {
                fpb.finish_and_clear();
            }
        }

        if let Some(master) = master {
            master.finish_and_clear();
        }

        let modules = Arc::new(asts.clone().into_iter().collect::<HashMap<_, _>>());

        Ok((asts.into_iter().map(|it| it.1).collect(), modules))
    }

    fn create_ast<'a>(&'a self, namespace: &'a str, file: PathBuf, _keep: bool) -> Result<AST<'a>> {
        let file_name = self.file_names.get(&file).ok_or(Error::Basic(format!(
            "Failed to find name for file: {}",
            file.display()
        )))?;

        let module = self.modules.get(&file).ok_or(Error::Basic(format!(
            "Failed to find module name for file: {}",
            file.display()
        )))?;

        let data = self.files.get(&file).ok_or(Error::Basic(format!(
            "Failed to read file: {}",
            file.display()
        )))?;

        let dump_dir = self.out_dir.join(".dpscript");
        let parent = file.parent().unwrap();
        let mut parts = Vec::new();
        let mut found = false;

        parts.push(namespace.into());

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

        if !dump_dir.exists() && self.dump_ast {
            fs::create_dir_all(&dump_dir)?;
        }

        let ast = FileParser::parse(file_name, module, namespace, &data)?;

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
