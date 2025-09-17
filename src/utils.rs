use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use pulldown_cmark::Options;

pub fn create_options() -> Options {
    Options::ENABLE_FOOTNOTES | Options::ENABLE_WIKILINKS
}

/// Create HashSet of canonicalized paths from vector of paths
pub fn create_file_set(vec_files: &Vec<PathBuf>) -> HashSet<PathBuf> {
    vec_files
        .iter()
        .filter_map(|s| fs::canonicalize(s).ok())
        .collect()
}

/// Return a path relative to current working directory
pub fn relative_path(target: &Path) -> String {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    pathdiff::diff_paths(target, cwd)
        .unwrap_or_else(|| target.to_path_buf())
        .display()
        .to_string()
}

/// Return a Vec where each entry is the byte offset of the start of a line
pub fn compute_line_starts(text: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(
            text.char_indices()
                .filter_map(|(i, c)| (c == '\n').then_some(i + 1)),
        )
        .collect()
}

/// Convert a byte offset into (line, column) given precomputed line starts
pub fn offset_to_line_col(offset: usize, line_starts: &[usize]) -> (usize, usize) {
    match line_starts.binary_search(&offset) {
        Ok(line) => (line + 1, 1), // exact match, first col
        Err(insert_point) => {
            let line = insert_point - 1;
            let col = offset - line_starts[line] + 1;
            (line + 1, col)
        }
    }
}
