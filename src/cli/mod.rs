//! Command-line syntax. Compilation and source loading live in their own layers.
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rusk",
    version,
    about = "Compile, format, and inspect Rusk programs",
    arg_required_else_help = true
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Generate C++20 source or dump an intermediate representation
    #[command(args_override_self = true)]
    Compile(CompileArgs),
    /// Format Rusk source without type checking or following includes
    Fmt(FormatArgs),
    /// List the Rusk sources and C++ resources loaded by a program
    Dependencies {
        /// Rusk source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

#[derive(Args)]
pub(crate) struct FormatArgs {
    /// Source files (multiple files require --check or --write)
    #[arg(value_name = "FILE", required = true, num_args = 1..)]
    pub files: Vec<PathBuf>,
    /// Fail if any source needs formatting; do not write files
    #[arg(long, conflicts_with = "write")]
    pub check: bool,
    /// Replace source files with their formatted contents
    #[arg(long)]
    pub write: bool,
}

impl FormatArgs {
    pub fn validate(&self) -> Result<(), clap::Error> {
        if !self.check && !self.write && self.files.len() != 1 {
            return Err(Cli::command().error(
                clap::error::ErrorKind::WrongNumberOfValues,
                "fmt: multiple files require --check or --write",
            ));
        }
        Ok(())
    }
}

#[derive(Args)]
pub(crate) struct CompileArgs {
    /// Rusk source file
    #[arg(value_name = "FILE")]
    pub file: PathBuf,
    /// Output file (the default applies to every emission stage)
    #[arg(short, long, value_name = "OUTPUT", default_value = "a.cpp")]
    pub output: PathBuf,
    /// Disable backend optimizations for comparison
    #[arg(long)]
    pub no_opt: bool,
    /// Representation to emit
    #[arg(long, value_enum, value_name = "STAGE", default_value = "cpp")]
    pub emit: EmitStage,
}

#[derive(Clone, Copy, ValueEnum)]
pub(crate) enum EmitStage {
    Cst,
    Ast,
    Hir,
    Lir,
    Cpp,
}
