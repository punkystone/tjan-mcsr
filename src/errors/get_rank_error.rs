use std::string::FromUtf8Error;

use hyper::http::uri::InvalidUri;
#[derive(Debug)]
pub enum GetRankError {
    InvalidUrl,
    RequestError,
    Utf8Error,
    ParseError,
}

impl From<InvalidUri> for GetRankError {
    fn from(_: InvalidUri) -> Self {
        GetRankError::InvalidUrl
    }
}

impl From<hyper::Error> for GetRankError {
    fn from(_: hyper::Error) -> Self {
        GetRankError::RequestError
    }
}

impl From<FromUtf8Error> for GetRankError {
    fn from(_: FromUtf8Error) -> Self {
        GetRankError::Utf8Error
    }
}

impl From<serde_json::Error> for GetRankError {
    fn from(_: serde_json::Error) -> Self {
        GetRankError::ParseError
    }
}

impl std::fmt::Display for GetRankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetRankError::InvalidUrl => write!(f, "mcsr invalid request url"),
            GetRankError::RequestError => write!(f, "mcsr request error"),
            GetRankError::Utf8Error => write!(f, "utf8 parse error"),
            GetRankError::ParseError => write!(f, "mcsr response parse error"),
        }
    }
}
