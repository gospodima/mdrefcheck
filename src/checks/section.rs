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
