use crate::error::AppError;
use std::path::Path;

pub fn read_file(path: &str) -> Result<String, AppError> {
    std::fs::read_to_string(path).map_err(AppError::Io)
}

pub fn write_file(path: &str, content: &str) -> Result<(), AppError> {
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content).map_err(AppError::Io)
}

pub fn rename_file(old_path: &str, new_path: &str) -> Result<(), AppError> {
    std::fs::rename(old_path, new_path).map_err(AppError::Io)
}

pub fn count_words(content: &str) -> i64 {
    content.split_whitespace().count() as i64
}
