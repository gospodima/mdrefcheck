use colored::Colorize;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::utils::relative_path;

/// Gather markdown files from paths (file or dir)
#[must_use]
pub fn gather_markdown_files<S: ::std::hash::BuildHasher>(
    paths: &[PathBuf],
    exclude: &HashSet<PathBuf, S>,
) -> Vec<PathBuf> {
    paths
        .iter()
        .flat_map(|path| {
            if let Ok(canonical) = fs::canonicalize(path) {
                collect_markdown_from_path(&canonical, exclude)
            } else {
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
fn collect_markdown_from_path<S: ::std::hash::BuildHasher>(
    path: &Path,
    exclude: &HashSet<PathBuf, S>,
) -> Vec<PathBuf> {
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
                    .is_ok_and(|p| !exclude.contains(&p))
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
    path.is_file() && path.extension().is_some_and(|ext| ext == "md")
}
