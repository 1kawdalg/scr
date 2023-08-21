//! All parsing <i>structures</i> and <i>modules</i>
use reqwest::{blocking};
use scraper::{ElementRef, Html, Selector, error::SelectorErrorKind};
use crate::error::{ScrapingErrorKind, ScrapingHttpErrorKind};

/// create new instance
fn instance(url: &str) -> Result<Scraper, ScrapingHttpErrorKind> {
    let document = blocking::get(url)
        .map_err(|_| ScrapingHttpErrorKind::GettingDataError)?
        .text()
        .map_err(|_| ScrapingHttpErrorKind::GettingDataError)?;

    Ok(Scraper { document: Html::parse_document(&document) })
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
    pub fn new(url: &str) -> Result<Scraper, ScrapingHttpErrorKind> {
        instance(format!("https://{url}").as_str())
    }

    /// create a new instance of scraper <i>using a site code **(without http://)**</i>
    pub fn from_http(url: &str) -> Result<Scraper, ScrapingHttpErrorKind> {
        instance(format!("http://{url}").as_str())
    }

    /// create a new instance of scraper <i>using a **fragment** of a site code</i>
    pub fn from_fragment(fragment: &str) -> Result<Scraper, SelectorErrorKind> {
        Ok(Scraper { document: Html::parse_fragment(fragment) })
    }

    /// get elements using selector
    pub fn get_els<'a>(&'a self, sel: &'a str) -> Result<Vec<ElementRef>, ScrapingErrorKind> {
        let parsed_selector = Selector::parse(sel).map_err(|_| ScrapingErrorKind::InvalidSelector)?;

        let elements = self.document
            .select(&parsed_selector)
            .collect::<Vec<ElementRef>>();

        Ok(elements)
    }

    /// get an element using selector
    pub fn get_el<'a>(&'a self, sel: &'a str) -> Result<ElementRef, ScrapingErrorKind> {
        let element = *self.get_els(sel)?
            .get(0)
            .ok_or(ScrapingErrorKind::NotFound)?;

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
    pub fn get_text_once(&self, sel: &str) -> Result<String, ScrapingErrorKind> {
        let text = self.get_el(sel)?
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
    pub fn get_all_text(self, sel: &str) -> Result<Vec<String>, ScrapingErrorKind> {
        let text = self.get_els(sel)?
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
    pub fn get_attr_once<'a>(&'a self, sel: &'a str, attr: &'a str) -> Result<&str, ScrapingErrorKind> {
        let attr = self.get_el(sel)?
            .value()
            .attr(attr)
            .ok_or(ScrapingErrorKind::NotFound)?;

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
    pub fn get_all_attr<'a>(&'a self, sel: &'a str, attr: &str) -> Result<Vec<&str>, SelectorErrorKind> {
        let attrs = self.get_els(sel)
            .unwrap()
            .iter()
            .map(|element| element.value().attr(attr).expect("Some elements do not contain the desired attribute"))
            .collect();

        Ok(attrs)
    }
}
