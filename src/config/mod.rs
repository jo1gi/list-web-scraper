/// Structs for storing config for sites
mod site;
pub mod lua;
#[cfg(test)]
mod test;

pub use site::{Site, Element, Function};
use crate::error::SpanreedError;
use std::path::PathBuf;
use serde::Deserialize;

/// Configuration
#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    /// List of sites to scrape
    pub sites: Vec<Site>,
}

impl Config {

    /// Creates a new config object based on yaml file
    pub fn new(name: &str) -> Result<Self, SpanreedError> {
        let path = find_config(name, "yaml")?;
        let data = std::fs::read_to_string(path)?;
        return Ok(serde_yaml::from_str(&data)?);
    }

}

/// Tries to find the config file based on the name. Starts by looking in a relative path and
/// afterwards tries to find it in the users config directory.
pub fn find_config(name: &str, ext: &str) -> Result<PathBuf, SpanreedError> {
    // Relative path
    let mut relative = PathBuf::new();
    relative.push(name);
    if relative.is_file() {
        return Ok(relative);
    }
    // Config directory
    let mut config = match dirs::config_dir() {
        Some(dir) => dir,
        None => return Err(SpanreedError::ConfigNotFound(String::from(name))),
    };
    config.push(env!("CARGO_CRATE_NAME"));
    config.push(name);
    config.set_extension(ext);
    if config.is_file() {
        return Ok(config);
    }
    return Err(SpanreedError::ConfigNotFound(String::from(name)));
}
