pub mod http;

use thiserror::Error;

/// Contains the consensus on both v4 and v6 external IPs.

pub type Result<T> = std::result::Result<T, SourceError>;

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("Failed to retrieve request result")]
    RequestError(#[from] reqwest::Error),
    #[error("Missing field `{0}` in response")]
    JsonFieldMissing(&'static str),
    #[error("Malformed field `{0}` in response: `{1}`")]
    JsonFieldMalformed(&'static str, String),
    #[error("Malformed raw IP value `{0}`")]
    RawIpMalformed(#[from] std::net::AddrParseError),
}
