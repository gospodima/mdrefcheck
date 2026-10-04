use std::{fs, path::Path, sync::Arc};

use crate::parser;
use crate::scanner::is_markdown;
use crate::utils::split_dest;

pub fn validate_section_link(
    current_path: &Path,
    dest: &str,
    section_links: &Arc<parser::SectionLinkMap>,
) -> Result<(), String> {
    // Raw file part kept for messages so users recognize their link
    let file_part = dest.split(['#', '?']).next().unwrap_or_default();
    let (path, heading_part) = split_dest(dest);

    let target_file = if path.is_empty() {
        current_path.to_path_buf()
    } else {
        let resolved = current_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(&path);
        fs::canonicalize(&resolved)
            .map_err(|_| format!("File not found: {file_part}"))?
    };

    // Empty fragment (`#`) links to the top of the page; anchors into
    // non-Markdown files (e.g. `script.py#L10`) are not headings, so skip them
    if let Some(heading) = heading_part.filter(|h| !h.is_empty())
        && is_markdown(&target_file)
        && !section_links
            .entry(target_file.clone())
            .or_try_insert_with(|| parser::parse_file_headings(&target_file))
            .map_err(|e| format!("Cannot read {file_part}: {e}"))?
            .contains(&heading)
    {
        return Err(format!(
            "Missing heading #{heading}{}",
            if file_part.is_empty() {
                String::new()
            } else {
                format!(" in {file_part}")
            }
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::Arc;
    use tempfile::tempdir;

    #[test]
    fn validate_existing_and_missing_headings() {
        let dir = tempdir().unwrap();
        let cur = dir.path().join("cur.md");
        let tgt = dir.path().join("tgt.md");

        fs::write(&cur, "[link](tgt.md#intro)").unwrap();
        fs::write(&tgt, "# Intro\n## Other").unwrap();

        let map = Arc::new(parser::SectionLinkMap::new());

        // valid
        assert!(validate_section_link(&cur, "tgt.md#intro", &map).is_ok());

        // missing heading
        let err = validate_section_link(&cur, "tgt.md#missing", &map);
        assert!(err.is_err());
        assert!(err.err().unwrap().contains("Missing heading"));

        // missing file
        let err2 = validate_section_link(&cur, "nope.md#h", &map);
        assert!(err2.is_err());
        assert!(err2.err().unwrap().contains("File not found"));

        // unreadable markdown target reports error instead of panicking
        fs::write(dir.path().join("bin.md"), [0xff, 0xfe]).unwrap();
        let err3 = validate_section_link(&cur, "bin.md#h", &map);
        assert!(err3.err().unwrap().contains("Cannot read bin.md"));

        // anchors into non-markdown files are not checked
        fs::write(dir.path().join("script.py"), "print()").unwrap();
        assert!(validate_section_link(&cur, "script.py#L10", &map).is_ok());
    }
}
