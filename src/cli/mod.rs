use crate::{Result, compiler::Compiler};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Build a project
    #[clap(aliases = ["b"])]
    Build {
        #[arg(short, long = "config", default_value_os_t = PathBuf::from("./pack.toml"))]
        config_path: PathBuf,

        #[arg(short, long = "output")]
        out_dir: Option<PathBuf>,

        #[arg(short = 'A', long)]
        dump_ast: bool,

        #[arg(short = 'T', long)]
        dump_tokens: bool,

        #[arg(short = 'I', long)]
        dump_ir: bool,

        #[arg(
            long,
            help = "Build manually-created IR files. THIS IS ONLY FOR DEBUG!! ERRORS HERE WILL BE HARD TO DEBUG!"
        )]
        build_ir: bool,
    },

    /// Compile a single file
    #[clap(aliases = ["c"])]
    Compile {
        file: PathBuf,

        #[arg(short, long = "output", default_value_os_t = PathBuf::from("."))]
        out_dir: PathBuf,

        #[arg(short = 'A', long)]
        dump_ast: bool,

        #[arg(short = 'T', long)]
        dump_tokens: bool,
    },
}

impl Cli {
    pub fn start() -> Result<()> {
        Self::parse().run()
    }

    pub fn run(&self) -> Result<()> {
        self.command.run()
    }
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Build {
                config_path,
                dump_ast,
                dump_tokens,
                dump_ir,
                out_dir,
                build_ir,
            } => {
                Compiler::new(
                    config_path.clone(),
                    out_dir.clone(),
                    *build_ir,
                    *dump_tokens,
                    *dump_ast,
                    *dump_ir,
                )?
                .compile_project()?;
            }

            Self::Compile {
                file: _,
                dump_ast: _,
                dump_tokens: _,
                out_dir: _,
            } => {
                todo!("soon(TM)")
            }
        }

        Ok(())
    }
}
