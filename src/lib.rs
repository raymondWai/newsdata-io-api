use std::fmt::{self, Display, Formatter};
use ureq::serde_json;

pub mod apis;
pub mod newsdata_io;
pub use newsdata_io::NewsdataIO;

pub type Json = serde_json::Value;
pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// An Error returned by the API
    ApiError(String),
    /// An Error not related to the API
    RequestError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ApiError(msg) => write!(f, "API error: {}", msg),
            Error::RequestError(msg) => write!(f, "Request error: {}", msg),
        }
    }
}
