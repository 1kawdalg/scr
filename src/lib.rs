//! # This crate is just parse documents and load files
//!
//! ## Example functions:
//! ```
//! use scr::Scraper;
//! use scr::FileLoader;
//!
//! fn parse_site() {
//!     let scraper = Scraper::new("scrapeme.live/shop/").unwrap();
//!     let element = scraper.get_el("main#main>ul>li.product>a>h2").unwrap();
//!
//!     assert_eq!(element.inner_html(), "Bulbasaur")
//! }
//!
//! fn parse_site_fragment() {
//!     let scraper = Scraper::new("scrapeme.live/shop/").unwrap();
//!     let fragment = scraper.get_text_once("main#main>ul>li.product>a").unwrap();
//!     let new_scraper = Scraper::from_fragment(fragment.as_str()).unwrap();
//!     let element = new_scraper.get_el("a").unwrap();
//!
//!     assert_eq!(element.inner_html(), "Bulbasaur")
//! }
//!
//! fn load_file() {
//!     let file_loader = FileLoader::new(
//!         "scrapeme.live/wp-content/uploads/2018/08/011.png",
//!         "./data/some_png.png"
//!     ).unwrap();
//!
//!     assert_eq!(
//!         file_loader.file
//!             .file_name().unwrap()
//!             .to_str().unwrap(),
//!         "some_png.png"
//!     );
//! }
//!
//! parse_site();
//! parse_site_fragment();
//! load_file();
//! ```
pub mod fileloader;
pub mod scraping;
pub mod error;

pub use fileloader::FileLoader;
pub use scraping::Scraper;
