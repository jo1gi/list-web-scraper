use serde::{Deserialize};
use std::collections::HashMap;

/// Information about how a site should be scraped
#[derive(Debug, PartialEq, Deserialize)]
pub struct Site {
    /// Name of site. This value is only used for logging.
    pub name: String,
    /// URL of the site to be scraped. This url can contain "{key}" which will be replaced by a key
    /// given by the user.
    pub url: String,
    /// Type of document to scrape.
    #[serde(default)]
    pub doctype: DocumentType,
    /// List of information to scrape from website.
    pub structure: HashMap<String, Element>,
    /// Http request headers
    pub headers: Option<HashMap<String, String>>,
}

/// Type of document to scrape
#[derive(Debug, PartialEq, Deserialize)]
pub enum DocumentType {
    Html,
    Json,
}

impl Default for DocumentType {
    fn default() -> Self { DocumentType::Html }
}

/// Information about which element and what information from that element should be scraped.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Element {
    /// Css selector for given element
    pub selector: Option<String>,
    /// Value to return instead of an items content
    pub value: Option<String>,
    /// Index in list of possible items
    #[serde(default)]
    pub index: usize,
    /// Value to get from the tag
    pub get: Option<String>,
    /// Prefix to add to the value
    pub prefix: Option<String>,
    /// Lua function to call
    pub function: Option<Function>,
    /// Trim output string
    pub trim: Option<bool>,
    pub remove: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Function {
    pub name: String,
    pub args: Option<Vec<Box<Element>>>,
}
