//! Error types for bazzounquester

use std::fmt;

/// Result type for bazzounquester operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur in bazzounquester
#[derive(Debug)]
pub enum Error {
    /// HTTP request error
    HttpRequest(reqwest::Error),

    /// Invalid header format
    InvalidHeader(String),

    /// Invalid query parameter format
    InvalidQuery(String),

    /// Invalid JSON body
    InvalidJson(serde_json::Error),

    /// IO error
    Io(std::io::Error),

    /// Readline error
    Readline(rustyline::error::ReadlineError),

    /// Invalid command
    InvalidCommand(String),

    /// Missing required argument
    MissingArgument(String),

    /// Unsupported HTTP method
    UnsupportedMethod(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpRequest(e) => write!(f, "HTTP request failed: {}", e),
            Error::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            Error::InvalidQuery(msg) => write!(f, "Invalid query parameter: {}", msg),
            Error::InvalidJson(e) => write!(f, "Invalid JSON: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Readline(e) => write!(f, "Readline error: {}", e),
            Error::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
            Error::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
            Error::UnsupportedMethod(method) => write!(f, "Unsupported HTTP method: {}", method),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpRequest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::InvalidJson(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(err: rustyline::error::ReadlineError) -> Self {
        Error::Readline(err)
    }
}
