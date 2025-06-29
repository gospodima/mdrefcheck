use colored::Colorize;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::utils::relative_path;

/// Gather markdown files from paths (file or dir)
pub fn gather_markdown_files(
    paths: &[PathBuf],
    exclude: &HashSet<PathBuf>,
) -> Vec<PathBuf> {
    paths
        .iter()
        .flat_map(|path| match fs::canonicalize(path) {
            Ok(canonical) => collect_markdown_from_path(&canonical, exclude),
            Err(_) => {
                eprintln!(
                    "{}",
                    format!("Skipping invalid path: {}", path.display()).yellow()
                );
                vec![]
            }
        })
        .collect()
}

/// Collect markdown file(s) from a path (file or dir)
fn collect_markdown_from_path(path: &Path, exclude: &HashSet<PathBuf>) -> Vec<PathBuf> {
    if exclude.contains(path) {
        eprintln!(
            "{}",
            format!(
                "Skipping directly specified and excluded path: {}",
                relative_path(path)
            )
            .yellow()
        );
        return vec![];
    }
    if is_markdown_file(path) {
        vec![path.to_path_buf()]
    } else if path.is_dir() {
        WalkDir::new(path)
            .into_iter()
            .filter_entry(|entry| {
                entry
                    .path()
                    .canonicalize()
                    .map_or(false, |p| !exclude.contains(&p))
            })
            .filter_map(Result::ok)
            .filter(|entry| is_markdown_file(entry.path()))
            .filter_map(|entry| fs::canonicalize(entry.path()).ok())
            .collect()
    } else {
        vec![]
    }
}

/// Determine if the given file path is a markdown file
fn is_markdown_file(path: &Path) -> bool {
    path.is_file() && path.extension().map_or(false, |ext| ext == "md")
}
