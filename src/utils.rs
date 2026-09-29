use percent_encoding::percent_decode_str;
use pulldown_cmark::Options;

#[must_use]
pub fn create_options() -> Options {
    Options::ENABLE_FOOTNOTES
        | Options::ENABLE_WIKILINKS
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
}

/// True for protocol-relative (`//host`) or scheme-prefixed (`tel:`, `HTTPS://`) links.
/// Schemes need 2+ chars so Windows drive letters (`C:`) stay local paths.
#[must_use]
pub fn is_external(dest: &str) -> bool {
    if dest.starts_with("//") {
        return true;
    }
    dest.split_once(':').is_some_and(|(scheme, _)| {
        scheme.len() >= 2
            && scheme.starts_with(|c: char| c.is_ascii_alphabetic())
            && scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
    })
}

/// Split a local link destination into percent-decoded (path, fragment), dropping `?query`
#[must_use]
pub fn split_dest(dest: &str) -> (String, Option<String>) {
    let (path, fragment) = dest
        .split_once('#')
        .map_or((dest, None), |(p, f)| (p, Some(f)));
    let path = path.split_once('?').map_or(path, |(p, _)| p);
    let decode = |s: &str| percent_decode_str(s).decode_utf8_lossy().into_owned();
    (decode(path), fragment.map(decode))
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

    #[test]
    fn test_create_options() {
        let opts = create_options();
        assert!(opts.contains(Options::ENABLE_FOOTNOTES));
        assert!(opts.contains(Options::ENABLE_WIKILINKS));
        assert!(opts.contains(Options::ENABLE_TASKLISTS));
    }

    #[test]
    fn test_is_external() {
        for d in ["https://x", "HTTPS://x", "tel:+123", "ftp://x", "//host/a"] {
            assert!(is_external(d), "{d}");
        }
        for d in ["a.md", "./a.md#h", "C:\\x.md", "#top"] {
            assert!(!is_external(d), "{d}");
        }
    }

    #[test]
    fn test_split_dest() {
        let s = |p: &str, f: Option<&str>| (p.to_string(), f.map(str::to_string));
        assert_eq!(split_dest("a%20b.png"), s("a b.png", None));
        assert_eq!(split_dest("a.png?raw=1"), s("a.png", None));
        assert_eq!(split_dest("a.md?q#h%C3%A9"), s("a.md", Some("hé")));
        assert_eq!(split_dest("#"), s("", Some("")));
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
