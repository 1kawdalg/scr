//! Load anything <i>(now only files)</i>

use std::{
    fs::File, io::{ Cursor, copy }
};

use crate::FileType;
use reqwest::blocking;

/// Load file
pub struct FileLoader {
    pub file: File
}

impl FileLoader {
    /// load file and save his <i>(please, write site url **without https://**)</i>
    ///
    /// Example:
    /// ```
    /// use scr::{ FileLoader, FileType };
    ///
    /// let file_loader = FileLoader::new(
    ///     "scrapeme.live/wp-content/uploads/2018/08/011.png",
    ///     "example",
    ///     FileType::Png
    /// );
    ///
    /// let file = file_loader.get_file();
    /// ```
    pub fn new(url: &str, file_name: &str, file_type: FileType) -> FileLoader {
        let file_end = match file_type {
            FileType::Json => "json",
            FileType::Png  => "png",
            FileType::Jpeg => "jpeg",
            FileType::Jpg  => "jpg",
            FileType::Xlsx => "xlsx",
            FileType::Txt  => "txt"
        };

        let response = blocking::get(format!("https://{url}"))
            .expect("Can not get site by url");

        let mut file = File::create(format!("../{file_name}.{file_end}"))
            .expect("Can not create file");

        let mut content = Cursor::new(response.bytes().expect("Can not get response body as bytes"));
        copy(&mut content, &mut file).expect("Can not copy content");

        FileLoader { file }
    }
}