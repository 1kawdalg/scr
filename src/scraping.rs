//! All parsing <i>structures</i> and <i>modules</i>
use reqwest::{blocking, Error};
use scraper::{ElementRef, Html, Selector, error::SelectorErrorKind};

/// create new instance
fn instance(url: &str) -> Result<Scraper, Error> {
    let response = blocking::get(url)
        .expect("Can not get site by url")
        .text()
        .expect("Can not get the response text");

    Ok(Scraper { document: Html::parse_document(&response) })
}

/// Simple parser
///
/// Example of using the received ElementRef:
/// ```
/// use scr::Scraper;
///
/// let scraper = Scraper::new("scrapeme.live/shop/")
///     .unwrap();
///
/// let element = scraper.get_el("main#main>ul>li.product>a>h2")
///     .unwrap();
///
/// assert_eq!(element.inner_html(), "Bulbasaur");
/// ```
pub struct Scraper {
    document: Html,
}

impl Scraper {
    /// create a new instance of scraper <i>using a site code **(without https://)**</i>
    pub fn new(url: &str) -> Result<Scraper, Error> {
        instance(format!("https://{url}").as_str())
    }

    /// create a new instance of scraper <i>using a site code **(without http://)**</i>
    pub fn from_http(url: &str) -> Result<Scraper, Error> {
        instance(format!("http://{url}").as_str())
    }

    /// create a new instance of scraper <i>using a **fragment** of a site code</i>
    pub fn from_fragment(fragment: &str) -> Result<Scraper, SelectorErrorKind> {
        Ok(Scraper { document: Html::parse_fragment(fragment) })
    }

    /// get elements using selector
    pub fn get_els(&self, sel: &str) -> Result<Vec<ElementRef>, SelectorErrorKind> {
        let elements = self.document
            .select(&Selector::parse(sel).expect("Can not parse"))
            .collect::<Vec<ElementRef>>();

        Ok(elements)
    }

    /// get an element using selector
    pub fn get_el(&self, sel: &str) -> Result<ElementRef, SelectorErrorKind> {
        let element = *self.get_els(sel)
            .unwrap()
            .get(0)
            .unwrap();

        Ok(element)
    }

    /// get text from an element
    ///
    /// Example:
    /// ```
    /// use scr::Scraper;
    ///
    /// let scraper = Scraper::new("scrapeme.live/shop/")
    ///     .unwrap();
    ///
    /// let text = scraper.get_text_once(
    ///     "main#main>ul>li.product>a>h2",
    /// ).unwrap();
    ///
    /// assert_eq!(text, "Bulbasaur")
    /// ```
    pub fn get_text_once(&self, sel: &str) -> Result<String, SelectorErrorKind> {
        let text = self.get_el(sel)
            .unwrap()
            .inner_html();

        Ok(text)
    }

    /// get text from all elements
    ///
    /// Example:
    /// ```
    /// use scr::Scraper;
    ///
    /// let scraper = Scraper::new("scrapeme.live/shop/")
    ///     .unwrap();
    ///
    /// let text = scraper.get_all_text(
    ///     "main#main>ul>li.product>a>h2",
    /// ).unwrap();
    ///
    /// assert_eq!(text.get(0).unwrap(), "Bulbasaur")
    /// ```
    pub fn get_all_text(&self, sel: &str) -> Result<Vec<String>, SelectorErrorKind> {
        let text = self.get_els(sel)
            .unwrap()
            .iter()
            .map(|element| element.inner_html())
            .collect();

        Ok(text)
    }

    /// get attribute from an element
    ///
    /// Example:
    /// ```
    /// use scr::Scraper;
    ///
    /// let scraper = Scraper::new("scrapeme.live/shop/")
    ///     .unwrap();
    ///
    /// let text = scraper.get_attr_once(
    ///     "main#main>ul>li.product>a>span",
    ///     "class",
    /// ).unwrap();
    ///
    /// assert_eq!(text, "price")
    /// ```
    pub fn get_attr_once(&self, sel: &str, attr: &str) -> Result<&str, SelectorErrorKind> {
        let attr = self.get_el(sel)
            .unwrap()
            .value()
            .attr(attr)
            .expect("Can not do");

        Ok(attr)
    }

    /// get attribute from all elements
    ///
    /// Example:
    /// ```
    /// use scr::Scraper;
    ///
    /// let scraper = Scraper::new("scrapeme.live/shop/")
    ///     .unwrap();
    ///
    /// let text = scraper.get_all_attr(
    ///     "main#main>ul>li.product>a>span",
    ///     "class",
    /// ).unwrap();
    ///
    /// assert_eq!(*text.get(0).unwrap(), "price")
    /// ```
    pub fn get_all_attr(&self, sel: &str, attr: &str) -> Result<Vec<&str>, SelectorErrorKind> {
        let attrs = self.get_els(sel)
            .unwrap()
            .iter()
            .map(|element| element.value().attr(attr).expect("Can not do"))
            .collect();

        Ok(attrs)
    }
}
