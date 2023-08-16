//! All parsing <i>structures</i> and <i>modules</i>

use crate::ItemNum;
use reqwest::blocking;
use scraper::{ Selector, Html, ElementRef };

/// Simple parser
///
/// Example of using the received ElementRef:
/// ```
/// use scr::Scraper;
///
/// let scraper = Scraper::new("scrapeme.live/shop/");
/// let element = scraper.get_el("main#main>ul>li.product>a>h2");
///
/// assert_eq!(element.inner_html(), "Bulbasaur")
/// ```
pub struct Scraper {
    document: Html
}

impl Scraper {
    /// create a new instance of scraper <i>using a site code **(without https://)**</i>
    pub fn new(url: &str) -> Scraper {
        let response = blocking::get(format!("https://{}", url))
            .expect("Can not get site by url")
            .text()
            .expect("Can not get the response text");

        Scraper { document: Html::parse_document(&response) }
    }

    /// create a new instance of scraper <i>using a **fragment** of a site code</i>
    pub fn from_fragment(fragment: &str) -> Scraper {
        Scraper { document: Html::parse_fragment(fragment) }
    }

    /// get elements using selector
    pub fn get_els(&self, sel: &str) -> Vec<ElementRef> {
        let mut vector = Vec::<ElementRef>::new();

        let items;
        let selector;

        selector = Self::get_sel(sel);
        items = self.document.select(&selector);

        for item in items {
            vector.push(item);
        }

        vector
    }

    /// get an element using selector
    pub fn get_el(&self, sel: &str) -> ElementRef {
        let vector = self.get_els(sel);

        *vector.get(0).expect("Can not get element")
    }

    /// get text from (an) element(s)
    ///
    /// Example:
    /// ```
    /// use scr::{ Scraper, ItemNum };
    /// let scraper = Scraper::new("scrapeme.live/shop/");
    ///
    /// let text = scraper.get_text(
    ///     "main#main>ul>li.product>a>h2",
    ///     ItemNum::Once
    /// );
    ///
    /// assert_eq!(text[0], "Bulbasaur")
    /// ```
    pub fn get_text(&self, sel: &str, item_num: ItemNum) -> Vec<String> {
        let mut new_vector = Vec::<String>::new();

        let vector = match item_num {
            ItemNum::Once =>
                vec![self.get_el(sel)],
            ItemNum::All =>
                self.get_els(sel)
        };

        for item in vector {
            new_vector.insert(
                0, item.inner_html()
            );
        }

        new_vector
    }

    /// get attribute from (an) element(s)
    ///
    /// Example:
    /// ```
    /// use scr::{ Scraper, ItemNum };
    ///
    /// let scraper = Scraper::new("scrapeme.live/shop/");
    ///
    /// let text = scraper.get_attr(
    ///     "main#main>ul>li.product>a>span",
    ///     "class",
    ///     ItemNum::Once
    /// );
    ///
    /// assert_eq!(text[0], "price")
    /// ```
    pub fn get_attr(&self, sel: &str, attr: &str, _type: ItemNum) -> Vec<String> {
        let mut new_vector = Vec::<String>::new();

        let vector = match _type {
            ItemNum::Once =>
                vec![self.get_el(sel)],
            ItemNum::All =>
                self.get_els(sel)
        };

        for item in vector {
            new_vector.insert(
                0, item.value().attr(attr).unwrap().to_string()
            );
        }

        new_vector
    }
    fn get_sel(sel: &str) -> Selector {
        Selector::parse(sel).unwrap()
    }
}