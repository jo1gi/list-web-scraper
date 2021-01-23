/// Error for Spanreed
#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum SpanreedError {
    /// Configuration file "{0}" could not be found
    ConfigNotFound(String),
    /// Could not parse given config file
    ConfigParsingError(#[from] serde_yaml::Error),
    /// IO Error
    IOError(#[from] std::io::Error),
    /// Invalid argument given - {0}
    WrongInputFormat(String),
    /// Networking error occured
    ReqwestError(#[from] reqwest::Error),
    /// Lua Execution Error
    LuaError(#[from] mlua::Error),
}
