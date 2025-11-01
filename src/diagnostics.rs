use colored::Colorize;
use std::path::Path;

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
            path: path.display().to_string(),
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
