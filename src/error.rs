#[derive(Debug)]
pub enum ScrapingErrorKind {
    NotFound,
    InvalidSelector
}

#[derive(Debug)]
pub enum ScrapingHttpErrorKind {
    GettingDataError
}