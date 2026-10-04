use std::path::Path;

use crate::utils::{is_external, split_dest};

pub fn validate_image(current_path: &Path, dest: &str) -> Result<(), String> {
    if is_external(dest) {
        return Ok(());
    }

    let resolved = current_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(split_dest(dest).0);

    if resolved.exists() {
        Ok(())
    } else {
        Err(format!("Image not found: {dest}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn image_local_found_and_missing() {
        let dir = tempdir().unwrap();
        let cur = dir.path().join("cur.md");
        fs::write(&cur, "# hi").unwrap();

        let img = dir.path().join("img.png");
        fs::write(&img, "data").unwrap();

        // relative to current file
        assert!(validate_image(&cur, "img.png").is_ok());

        // missing
        assert!(validate_image(&cur, "missing.png").is_err());
    }
}
