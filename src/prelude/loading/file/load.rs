//! Different ways of load file
use super::*;

/// Just load file! (and get his wrapper)
pub fn load_file(url: &str, file_name: &str, file_type: FileType) -> FileLoader {
    FileLoader::new(url, file_name, file_type)
}