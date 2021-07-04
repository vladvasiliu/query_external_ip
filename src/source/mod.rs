//! Generic IP Source related items
pub mod http;

use thiserror::Error;

/// The result from a Source
pub type Result<T> = std::result::Result<T, SourceError>;

/// An error describing what when wrong while querying an IP source.
#[derive(Error, Debug)]
pub enum SourceError {
    /// A `reqwest` error while retrieving the external IP from a Source
    #[error("Failed to retrieve request result")]
    RequestError(#[from] reqwest::Error),
    /// The expected field name was not found in the JSON response
    #[error("Missing field `{0}` in response")]
    JsonFieldMissing(&'static str),
    /// The JSON field contained an invalid value
    #[error("Malformed field `{0}` in response: `{1}`")]
    JsonFieldMalformed(&'static str, String),
    /// Failed to parse the response
    #[error("Malformed raw IP value `{0}`")]
    RawIpMalformed(#[from] std::net::AddrParseError),
}
