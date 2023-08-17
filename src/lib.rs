//! # This crate is just parse documents and load files
//!
//! ## Example functions:
//! ```
//! use scr::Scraper;
//! use scr::FileLoader;
//!
//! fn parse_site() {
//!     let scraper = Scraper::new("scrapeme.live/shop/");
//!     let element = scraper.get_el("main#main>ul>li.product>a>h2");
//!
//!     assert_eq!(element.inner_html(), "Bulbasaur")
//! }
//!
//! fn parse_site_fragment() {
//!     let scraper = Scraper::new("scrapeme.live/shop/");
//!     let fragment = scraper.get_text_once("main#main>ul>li.product>a");
//!     let new_scraper = Scraper::from_fragment(fragment.as_str());
//!     let element = new_scraper.get_el("a");
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
//! ```
pub mod fileloader;
pub mod scraping;

pub use fileloader::FileLoader;
pub use scraping::Scraper;
