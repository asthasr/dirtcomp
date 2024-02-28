use clap::{arg, command, Parser, Subcommand};

mod commands;
mod config;
mod file_handling;
mod traversal;

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        name = "check",
        about = "Check a single base glob and target glob",
        visible_alias = "c"
    )]
    SingleCheck {
        #[clap()]
        base: glob::Pattern,

        #[clap()]
        target: glob::Pattern,
    },

    #[command(
        name = "multi-check",
        about = "Check multiple base and target globs",
        visible_alias = "cc"
    )]
    MultiCheck {
        #[arg(short = 'b', long = "base", required = true)]
        bases: Vec<glob::Pattern>,

        #[arg(short = 't', long = "target", required = true)]
        targets: Vec<glob::Pattern>,
    },
}

impl Commands {
    fn dispatch(args: Cli) {
        let filter_opts = (&args).into();

        match args.command {
            Commands::SingleCheck { base, target } => {
                commands::simple_check(filter_opts, base, target);
            }
            Commands::MultiCheck { bases, targets } => {
                commands::multi_check(filter_opts, &bases, &targets);
            }
        }
    }
}

/// This implementation allows ergonomic command-line representations of settings as _flags_ while
/// preserving positive-biased conditionals in the code.
impl From<&Cli> for config::FilterOpts {
    fn from(value: &Cli) -> Self {
        Self {
            include_files: !value.ignore_files,
            include_dirs: !value.ignore_dirs,
            symlinks: value.symlinks,
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    version,
    name = "dirtcomp",
    about = "Compare modification times between file globs",
    long_about = "Given two \
          (or more) file globs, this utility checks whether the target globs include any newer files \
          than the base globs. If they do, the utility exits with a successful status code (0). \
          If not, an unsuccessful status code (1) is emitted. \
          \n\n\
          This is motivated as a way to detect whether compilation is required by comparing a base \
          directory (source) with a target directory (build)."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[clap(
        long,
        global = true,
        default_value = "false",
        help = "Ignore the modification time of files"
    )]
    ignore_files: bool,

    #[clap(
        long,
        global = true,
        default_value = "false",
        help = "Ignore the modification time of directories"
    )]
    ignore_dirs: bool,

    #[clap(
        long,
        global = true,
        default_value = "traverse",
        help = "Ignore symlinks, traverse them, or use them as files"
    )]
    symlinks: config::SymlinkOption,
}

fn main() {
    let args = Cli::parse();
    Commands::dispatch(args);
}
