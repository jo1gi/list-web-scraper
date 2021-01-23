/// Returns config file retrieved from stdin
pub fn get_config() -> Result<String, SpanreedError> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer)?;
    return buffer;
}
