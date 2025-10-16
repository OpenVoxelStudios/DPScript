use crate::{Result, compiler::Compiler};
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::{env::set_current_dir, path::PathBuf};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,

    #[arg(short = 'C', long)]
    pub cwd: Option<PathBuf>,
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

        #[arg(short = 'D', long)]
        allow_dead_code: bool,

        #[arg(short, long)]
        detailed: bool,
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

        #[arg(short, long)]
        detailed: bool,
    },
}

impl Cli {
    pub fn start() -> Result<()> {
        Self::parse().run()
    }

    pub fn run(self) -> Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .with_max_level(self.verbosity)
            .init();

        if let Some(cwd) = &self.cwd {
            set_current_dir(cwd)?;
        }

        self.command.run(self.verbosity.is_silent())
    }
}

impl Commands {
    pub fn run(self, quiet: bool) -> Result<()> {
        match self {
            Self::Build {
                config_path,
                dump_ast,
                dump_tokens,
                dump_ir,
                out_dir,
                allow_dead_code,
                detailed,
            } => {
                Compiler::new(
                    config_path,
                    out_dir,
                    dump_tokens,
                    dump_ast,
                    dump_ir,
                    allow_dead_code,
                    detailed,
                    quiet,
                )?
                .compile_project()?;
            }

            Self::Compile {
                file: _,
                dump_ast: _,
                dump_tokens: _,
                out_dir: _,
                detailed: _,
            } => {
                todo!("single-file compilation is soon(TM)")
            }
        }

        Ok(())
    }
}
