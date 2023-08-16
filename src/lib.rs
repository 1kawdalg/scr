//! This crate is just parse documents and load files
pub mod loading;
pub mod scraping;

extern crate reqwest;
extern crate scraper;

use reqwest::blocking as rb;

pub use loading::FileLoader;
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