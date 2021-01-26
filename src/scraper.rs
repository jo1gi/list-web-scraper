use crate::error::SpanreedError;
use crate::config::{Site, Element, Config, Function};
use std::collections::HashMap;
use futures::future;
use scraper::html::Html;
use scraper::Selector;
use scraper::element_ref::ElementRef;
use mlua::prelude::Lua;

/// Scrapes websites described in `config`
pub async fn scrape(config: Config, lua_env: &Lua) -> Result<Output, SpanreedError> {
    let mut output = Vec::new();
    let sites: Vec<_> = config.sites.iter()
        .map(|site| scrape_site(site, lua_env))
        .collect();
    for i in future::join_all(sites).await {
        output.extend(i);
    }
    return Ok(output);
}

/// Download website and parse it as a html document
async fn download_site(url: &str, headers: &Option<HashMap<String, String>>) -> Result<Html, SpanreedError> {
    let client = reqwest::Client::new();
    let mut req = client.get(url);
    if headers.is_some() {
        for (key, value) in headers.as_ref().unwrap() {
            req = req.header(key, value);
        }
    }
    let body = req.send()
        .await?
        .text()
        .await?;
    let tree = Html::parse_document(&body);
    return Ok(tree);
}

/// Finds all html elements in `tree` that matches the css selector in `element`
fn find_containers<'a>(tree: &'a Html, structure: &'a Element) -> Vec<ElementRef<'a>> {
    let selector = match &structure.selector {
        Some(s) => {
            match Selector::parse(s) {
                Ok(selector) => selector,
                Err(_) => return Vec::new(),
            }
        },
        None => return Vec::new(),
    };
    let mut containers: Vec<ElementRef> = tree.select(&selector).collect();
    if structure.remove.is_some() {
        let mut to_be_removed: Vec<usize> = Vec::new();
        for i in 0..containers.len() {
            let s = match Selector::parse(structure.remove.as_ref().unwrap()) {
                Ok(s) => s,
                Err(_) => return Vec::new(),
            };
            let c = containers[i];
            if c.select(&s).into_iter().count() > 0 {
                to_be_removed.push(i);
            }
        }
        for i in to_be_removed.iter().rev() {
            containers.remove(*i);
        }
    }
    return containers;
}

/// Tries to find `element` in `tree`
fn find_elements<'a>(tree: &'a ElementRef, structure: &'a Element) -> Option<Vec<ElementRef<'a>>> {
    let selector = match &structure.selector {
        Some(s) => {
            match Selector::parse(s) {
                Ok(selector) => selector,
                Err(_) => return None,
            }
        },
        None => return None,
    };
    return Some(tree.select(&selector).collect());
}

/// Runs user defined function from lua file with arguments from scraped website
fn run_lua_function(tree: &ElementRef, func: &Function, lua_env: &Lua) -> Result<String, SpanreedError> {
    let args = match &func.args {
        Some(args) => {
            args.iter()
                .filter_map(|arg| get_element(tree, arg, lua_env))
                .collect::<Vec<String>>()
        },
        None => Vec::new(),
    };
    let globals = lua_env.globals();
    let lua_func: mlua::Function = globals.get(func.name.clone())?;
    let result = lua_func.call::<_, String>(args)?;
    return Ok(result);
}

/// Find element and returns its given attribute
fn get_element(tree: &ElementRef, structure: &Element, lua_env: &Lua) -> Option<String> {
    // Returning data based on predefined value
    if structure.value.is_some() {
        return structure.value.clone();
    }
    // Returning data returned from a lua function
    if structure.function.is_some() {
        return run_lua_function(tree, &structure.function.as_ref().unwrap(), lua_env).ok();
    }
    // Returning data found on the downloaded site
    let elements = find_elements(tree, structure)?;
    if elements.len() <= structure.index {
        return None;
    }
    let element = elements[structure.index];
    let mut value: String = match &structure.get {
        Some(get) => {
            match element.value().attr(&get) {
                Some(value) => String::from(value),
                None => return None,
            }
        },
        None => element.text().collect(),
    };
    // Modifying value
    if structure.trim.is_some() && structure.trim.unwrap() {
        value = value.trim().to_string();
    }
    if structure.prefix.is_some() {
        value = structure.prefix.as_ref().unwrap().to_owned() + &value;
    }
    return Some(value);
}

/// Scrapes a single website described in `site`
async fn scrape_site(site: &Site, lua_env: &Lua) -> Vec<HashMap<String, String>> {
    let mut output = Vec::new();
    if !site.structure.contains_key("CONTAINER") {
        return output;
    }
    // Downlaoding site
    let tree = match download_site(&site.url, &site.headers).await {
        Ok(tree) => tree,
        Err(_) => return output,
    };
    // Finding containers
    let containers = find_containers(&tree, &site.structure.get("CONTAINER").unwrap());
    // Finding data in each container
    'outer: for container in containers {
        let mut attr = HashMap::new();
        for (key, value) in &site.structure {
            if key == "CONTAINER" {
                continue;
            }
            match get_element(&container, &value, lua_env) {
                Some(elem) => {attr.insert(key.clone(), elem);},
                None => continue 'outer,
            }
        }
        if attr.len() == 0 {
            continue;
        }
        output.push(attr);
    }
    return output;
}

/// Output format
pub type Output = Vec<HashMap<String, String>>;
