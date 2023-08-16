//! Different ways of load file
use crate::{ FileType, FileLoader };

/// Just load file! (and get his wrapper)
pub fn from_url(url: &str, file_name: &str, file_type: FileType) -> FileLoader {
    FileLoader::new(url, file_name, file_type)
}