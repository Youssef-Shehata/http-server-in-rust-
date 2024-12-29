use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid HTTP request: {0}")]
    InvalidRequest(String),

    #[error("Invalid HTTP method: {0}")]
    InvalidMethod(String),
}
