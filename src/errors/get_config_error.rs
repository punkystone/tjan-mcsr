use std::fmt::{Display, Formatter};

pub enum GetConfigError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
}

impl From<std::io::Error> for GetConfigError {
    fn from(error: std::io::Error) -> Self {
        GetConfigError::IoError(error)
    }
}

impl From<serde_json::Error> for GetConfigError {
    fn from(error: serde_json::Error) -> Self {
        GetConfigError::ParseError(error)
    }
}

impl Display for GetConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetConfigError::IoError(error) => write!(f, "IO Error: {error}"),
            GetConfigError::ParseError(error) => write!(f, "Parse Error: {error}"),
        }
    }
}
