//! Different ways of create Scraper object
use crate::Scraper;
use scraper::Html;

/// init Scraper using url <i>(without https://)</i>
pub fn from_url(url: &str) -> Scraper {
    Scraper::new(url)
}

pub fn from_fragment(fragment: &str) -> Scraper {
    Scraper {document: Html::parse_fragment(fragment)}
}