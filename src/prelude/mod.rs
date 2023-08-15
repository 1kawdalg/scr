//! In the prelude are the **main modules** of the <i>crate</i>
pub mod loading;
pub mod parsing;

use crate::{ rb, scraper };

pub use parsing::Scraper;
pub use loading::file::FileLoader;

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