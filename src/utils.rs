use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use pulldown_cmark::Options;

#[must_use]
pub fn create_options() -> Options {
    Options::ENABLE_FOOTNOTES | Options::ENABLE_WIKILINKS
}

/// Create ``HashSet`` of canonicalized paths from vector of paths
#[must_use]
pub fn create_file_set(vec_files: &[PathBuf]) -> HashSet<PathBuf> {
    vec_files
        .iter()
        .filter_map(|s| fs::canonicalize(s).ok())
        .collect()
}

/// Return a path relative to the current working directory
#[must_use]
pub fn relative_path(target: &Path) -> String {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Normalize target path first (fixes Windows \\?\ prefixes)
    let normalized =
        dunce::canonicalize(target).unwrap_or_else(|_| target.to_path_buf());

    pathdiff::diff_paths(&normalized, cwd)
        .unwrap_or(normalized)
        .display()
        .to_string()
}

/// Return a Vec where each entry is the byte offset of the start of a line
#[must_use]
pub fn compute_line_starts(text: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(
            text.char_indices()
                .filter_map(|(i, c)| (c == '\n').then_some(i + 1)),
        )
        .collect()
}

/// Convert a byte offset into (line, column) given precomputed line starts
#[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_options() {
        let opts = create_options();
        assert!(opts.contains(Options::ENABLE_FOOTNOTES));
        assert!(opts.contains(Options::ENABLE_WIKILINKS));
    }

    #[test]
    fn test_create_file_set_with_valid_paths() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "hello").unwrap();

        let vec_files = vec![file_path.clone()];
        let set = create_file_set(&vec_files);

        assert_eq!(set.len(), 1);
        assert!(set.contains(&fs::canonicalize(&file_path).unwrap()));
    }

    #[test]
    fn test_create_file_set_with_invalid_path() {
        let invalid = PathBuf::from("/nonexistent/path.txt");
        let set = create_file_set(&[invalid]);
        assert!(set.is_empty());
    }

    #[test]
    fn test_relative_path_within_cwd() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("foo.txt");
        fs::write(&file_path, "data").unwrap();

        let rel = relative_path(&file_path);
        assert!(rel.contains("foo.txt"));
    }

    // TODO: improve nonexistent file handling?
    #[test]
    fn test_relative_path_nonexistent_file() {
        let path = PathBuf::from("does_not_exist.txt");
        let rel = relative_path(&path);
        assert!(rel.contains("does_not_exist.txt"));
    }

    #[test]
    fn test_compute_line_starts() {
        let text = "line1\nline2\nline3";
        let starts = compute_line_starts(text);
        assert_eq!(starts, vec![0, 6, 12]);
    }

    #[test]
    fn test_offset_to_line_col_exact_match() {
        let text = "a\nb\nc";
        let starts = compute_line_starts(text);
        // offset 0 = line 1, col 1
        assert_eq!(offset_to_line_col(0, &starts), (1, 1));
        // offset 2 = start of line 2
        assert_eq!(offset_to_line_col(2, &starts), (2, 1));
    }

    #[test]
    fn test_offset_to_line_col_between_lines() {
        let text = "hello\nworld";
        let starts = compute_line_starts(text);
        // "world" starts at offset 6
        assert_eq!(offset_to_line_col(7, &starts), (2, 2)); // 'o' in world
    }

    #[test]
    fn test_offset_to_line_col_end_of_text() {
        let text = "abc";
        let starts = compute_line_starts(text);
        assert_eq!(offset_to_line_col(3, &starts), (1, 4)); // after last char
    }
}
