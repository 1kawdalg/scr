//! Load files
pub mod load;

use super::*;

/// Load file
pub struct FileLoader {
    file: File
}

impl FileLoader {
    /// load file and save his <i>(please, write site url **without https://**)</i>
    ///
    /// Example:
    /// ```
    /// use scr::prelude::{FileLoader, FileType};
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

        let resp = rb::get(format!("https://{url}"))
            .expect("Site is don't load");

        let mut file = File::create(format!("../{file_name}.{file_end}"))
            .expect("File don't make");

        let mut content = Cursor::new(resp.bytes().unwrap());
        copy(&mut content, &mut file).expect("File don't loaded");

        FileLoader { file }
    }

    /// file getter
    pub fn get_file(self) -> File {
        self.file
    }
}