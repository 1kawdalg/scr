<h1 align="center"><a href="https://docs.rs/scr/latest/scr/">scr</a></h1>

<p align="center">
  <a href="https://crates.io/crates/scr">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/scr?style=plastic&logo=rust">
  </a>
  <a href="https://crates.io/crates/scr">
    <img alt="Crates.io" src="https://img.shields.io/crates/d/scr?style=plastic">
  </a>
  <a href="https://github.com/1kawdalg/scr">
    <img alt="GitHub: 1kawdalg/scr" src="https://img.shields.io/github/languages/code-size/1kawdalg/scr?style=plastic&logo=github&label=1kawdalg%2Fscr">
  </a>
</p>

<blockquote cite="https://www.azquotes.com/quote/78518"><em>"Simplicity is prerequisite for reliability" — Edsger Dijkstra</em></blockquote>

## What is "scr"?
This is simplified fork of crates ```reqwest = {version = "0.11", features = ["blocking"]}```
and ```scraper = "0.17.1"``` which working together.
Also are system ```pub struct std::path::Path```, ```pub struct std::fs::File```
and ```pub fn std::fs::write```.

## "How use last stable version of scr in app?"
```toml
# Cargo.toml
[dependencies]
scr = "1.0.1"
```

## Examples
- ### parse site
```rust
use scr::Scraper;

fn main() {
    let scraper = Scraper::new("scrapeme.live/shop/").unwrap();
    let element = scraper.get_el("main#main>ul>li.product>a>h2").unwrap();

    assert_eq!(element.inner_html(), "Bulbasaur")
}
```
- ### parse fragment of site
```rust
use scr::Scraper;

fn main() {
    let scraper = Scraper::new("scrapeme.live/shop/").unwrap();
    let fragment = scraper.get_text_once("main#main>ul>li.product>a").unwrap();
    let new_scraper = Scraper::from_fragment(fragment.as_str()).unwrap();
    let element = new_scraper.get_el("a").unwrap();

    assert_eq!(element.inner_html(), "Bulbasaur")
}
```
- ### download file
```rust
use scr::FileLoader;

fn main() {
    let file_loader = FileLoader::new(
        "scrapeme.live/wp-content/uploads/2018/08/011.png",
        "./data/some_png.png"
    ).unwrap();
    
    assert_eq!(
        file_loader.file
            .file_name().unwrap()
            .to_str().unwrap(),
        "some_png.png"
    );
}
```

## The [crate](https://docs.rs/scr/latest/scr/) was developed by:
- ### version 1.0
[onekg](https://github.com/1kawdalg);\
[reloginn](https://github.com/reloginn)
