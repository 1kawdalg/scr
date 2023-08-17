//! Load anything <i>(now only files)</i>

use std::{fs, path::Path};

use reqwest::blocking;

/// Load file
pub struct FileLoader<'l> {
    pub file: &'l Path,
}

impl<'l> FileLoader<'l> {
    /// Simple file loader
    ///
    /// Example:
    /// ```
    /// use scr::FileLoader;
    ///
    /// let file_loader = FileLoader::new(
    ///     "scrapeme.live/wp-content/uploads/2018/08/011.png",
    ///     "./data/some_png.png"
    /// ).unwrap();
    ///
    /// assert_eq!(
    ///     file_loader.file
    ///         .file_name().unwrap()
    ///         .to_str().unwrap(),
    ///     "some_png.png"
    /// );
    /// ```
    pub fn new(url: &str, path: &'l str) -> Result<FileLoader<'l>, reqwest::Error> {
        let file = Path::new(path);

        if !file.exists() {
            fs::File::create(file).unwrap_or_else(|_| {
                fs::File::create(
                    std::env::current_dir()
                        .expect("Can not get current dir")
                        .join(file.file_name().expect("Can not get file name"))
                        .with_extension(file.extension().expect("Can not get extension")),
                )
                .expect("Can not create file")
            });
        }

        let response = blocking::get(format!("https://{url}"))?;
        let _ = fs::write(file, response.bytes()?);

        Ok(FileLoader { file })
    }
}
