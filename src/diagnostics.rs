use crate::utils::relative_path;
use colored::Colorize;
use std::path::Path;

/// Represents a markdown validation issue (Ruff-compatible output)
pub struct ValidationError {
    pub path: String,
    pub line: usize,
    pub col: usize,
    pub message: String,
}

impl ValidationError {
    pub fn new(
        path: &Path,
        line: usize,
        col: usize,
        message: impl Into<String>,
    ) -> Self {
        Self {
            path: relative_path(path),
            line,
            col,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}: {}",
            self.path.bold(),
            self.line,
            self.col,
            self.message
        )
    }
}
