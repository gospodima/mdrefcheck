use std::path::Path;

pub fn validate_image(current_path: &Path, dest: &str) -> Result<(), String> {
    if dest.starts_with("http://") || dest.starts_with("https://") {
        return Ok(());
    }

    let resolved = current_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(dest);

    if !resolved.exists() {
        Err(format!("Image not found: {}", dest))
    } else {
        Ok(())
    }
}
