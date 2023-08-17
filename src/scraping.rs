//! All parsing <i>structures</i> and <i>modules</i>
use reqwest::blocking;
use scraper::{ElementRef, Html, Selector};

/// Simple parser
///
/// Example of using the received ElementRef:
/// ```
/// use scr::Scraper;
///
/// let scraper = Scraper::new("scrapeme.live/shop/");
/// let element = scraper.get_el("main#main>ul>li.product>a>h2");
///
/// assert_eq!(element.inner_html(), "Bulbasaur");
/// ```
pub struct Scraper {
    document: Html,
}

impl Scraper {
    /// create a new instance of scraper <i>using a site code **(without https://)**</i>
    pub fn new(url: &str) -> Scraper {
        let response = blocking::get(format!("https://{}", url))
            .expect("Can not get site by url")
            .text()
            .expect("Can not get the response text");

        Scraper {
            document: Html::parse_document(&response),
        }
    }

    /// create a new instance of scraper <i>using a **fragment** of a site code</i>
    pub fn from_fragment(fragment: &str) -> Scraper {
        Scraper {
            document: Html::parse_fragment(fragment),
        }
    }

    /// get elements using selector
    pub fn get_els(&self, sel: &str) -> Vec<ElementRef> {
        self.document
            .select(&Selector::parse(sel).expect("Can not parse"))
            .collect()
    }

    /// get an element using selector
    pub fn get_el(&self, sel: &str) -> ElementRef {
        *self.get_els(sel).get(0).expect("Can not get an element")
    }

    /// get text from an element
    ///
    /// Example:
    /// ```
    /// use scr::Scraper;
    /// let scraper = Scraper::new("scrapeme.live/shop/");
    ///
    /// let text = scraper.get_text_once(
    ///     "main#main>ul>li.product>a>h2",
    /// );
    ///
    /// assert_eq!(text, "Bulbasaur")
    /// ```
    pub fn get_text_once(&self, sel: &str) -> String {
        self.get_el(sel).inner_html()
    }

    /// get text from all elements
    ///
    /// Example:
    /// ```
    /// use scr::Scraper;
    /// let scraper = Scraper::new("scrapeme.live/shop/");
    ///
    /// let text = scraper.get_all_text(
    ///     "main#main>ul>li.product>a>h2",
    /// );
    ///
    /// assert_eq!(text.get(0).unwrap(), "Bulbasaur")
    /// ```
    pub fn get_all_text(&self, sel: &str) -> Vec<String> {
        self.get_els(sel)
            .iter()
            .map(|element| element.inner_html())
            .collect()
    }

    /// get attribute from an element
    ///
    /// Example:
    /// ```
    /// use scr::Scraper;
    ///
    /// let scraper = Scraper::new("scrapeme.live/shop/");
    ///
    /// let text = scraper.get_attr_once(
    ///     "main#main>ul>li.product>a>span",
    ///     "class",
    /// );
    ///
    /// assert_eq!(text, "price")
    /// ```
    pub fn get_attr_once(&self, sel: &str, attr: &str) -> &str {
        self.get_el(sel).value().attr(attr).expect("Can not do")
    }

    /// get attribute from all elements
    ///
    /// Example:
    /// ```
    /// use scr::Scraper;
    ///
    /// let scraper = Scraper::new("scrapeme.live/shop/");
    ///
    /// let text = scraper.get_all_attr(
    ///     "main#main>ul>li.product>a>span",
    ///     "class",
    /// );
    ///
    /// assert_eq!(*text.get(0).unwrap(), "price")
    /// ```
    pub fn get_all_attr(&self, sel: &str, attr: &str) -> Vec<&str> {
        self.get_els(sel)
            .iter()
            .map(|element| element.value().attr(attr).expect("Can not do"))
            .collect()
    }
}
