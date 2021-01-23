use super::Config;
use std::path::PathBuf;

/// Test the values in the config file
fn test_config_contents(config: Config) {
    assert_eq!(config.sites[0].structure.get("CONTAINER").unwrap().selector.as_ref().unwrap(), "asd");
    assert_eq!(config.sites.len(), 1);
}

#[test]
/// Tests loading of a yaml config file
fn test_load_yaml_config() {
    let mut path = PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push("testdata");
    path.push("html_config.yaml");
    let s = path.as_os_str().to_str().unwrap();
    match Config::new(s) {
        Ok(config) => {
            test_config_contents(config);
        },
        Err(e) => panic!(e),
    }
}
