use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use regex::Regex;

/// CLI configuration for mdrefcheck
#[derive(Parser, Debug)]
#[command(name = "mdrefcheck", about = "Check markdown references.", version)]
pub struct CliConfig {
    /// Paths to check
    #[arg(required = true, value_name = "PATH")]
    pub paths: Vec<PathBuf>,

    /// Regex patterns to exclude from link validation
    #[arg(long, short, value_name = "REGEX")]
    pub ignore: Vec<Regex>,

    /// Paths to not check. Excluded files can be parsed though if they are referred.
    #[arg(long, short, value_name = "PATH")]
    pub exclude: Vec<PathBuf>,

    /// Disable standard ignore filters (gitignore, hidden files, etc.)
    #[arg(long)]
    pub no_ignore: bool,

    // -q, --quiet   (decreases verbosity)
    // -v, --verbose (increases verbosity, up to 2 times for 'trace')
    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
}
