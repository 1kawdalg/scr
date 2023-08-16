//! This crate is just parse documents and load files
pub mod fileloader;
pub mod scraping;

pub use fileloader::FileLoader;
pub use scraping::Scraper;

/// "**How many <i>values</i>** to consider?"
pub enum ItemNum {
    Once,
    All
}

/// Type of load file
pub enum FileType {
    Json,
    Png,
    Jpeg,
    Jpg,
    Xlsx,
    Txt
}