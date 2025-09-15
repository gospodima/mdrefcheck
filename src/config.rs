use std::path::PathBuf;

use clap::Parser;
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
    // /// Files to not check and parse.
    // #[arg(long, num_args = 1.., value_delimiter = ' ', value_name = "FILE")]
    // pub full_exclude_files: Vec<PathBuf>,
}
