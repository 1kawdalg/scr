//! This crate is just parse documents and load files
pub mod fileloader;
pub mod scraping;

pub use fileloader::FileLoader;
pub use scraping::Scraper;
