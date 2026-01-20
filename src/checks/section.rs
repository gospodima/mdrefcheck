use std::{fs, path::Path, sync::Arc};

use crate::parser;

pub fn validate_section_link(
    current_path: &Path,
    dest: &str,
    section_links: &Arc<parser::SectionLinkMap>,
) -> Result<(), String> {
    let (file_part, heading_part) = dest
        .split_once('#')
        .map_or((dest, None), |(f, h)| (f, Some(h)));

    let target_file = if file_part.is_empty() {
        current_path.to_path_buf()
    } else {
        let resolved = current_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(file_part);
        fs::canonicalize(&resolved)
            .map_err(|_| format!("File not found: {file_part}"))?
    };

    if let Some(heading) = heading_part
        && !section_links
            .entry(target_file.clone())
            .or_insert_with(|| parser::parse_file_headings(&target_file).unwrap())
            .contains(heading)
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
    }
}
