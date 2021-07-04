use super::{Result, SourceError};
use futures::stream::{self, StreamExt};
use futures::TryFutureExt;
use log::debug;
use once_cell::sync::Lazy;
use reqwest::{Client, Response, Url};
use std::fmt::Formatter;
use std::net::IpAddr;
use std::option::Option::None;
use std::time::Duration;

static HTTP_SOURCES: Lazy<Vec<HttpSource>> = Lazy::new(|| {
    [
        "https://icanhazip.com/",
        "https://myexternalip.com/raw",
        "https://ifconfig.io/ip",
        "https://ipecho.net/plain",
        "https://checkip.amazonaws.com/",
        "http://whatismyip.akamai.com/",
        "https://myip.dnsomatic.com/",
        "https://diagnostic.opendns.com/myip",
        "https://v4.ident.me/",
        "https://v6.ident.me/",
        "https://api4.ipify.org/",
        "https://api6.ipify.org/",
        "https://ipv4.wtfismyip.com/text",
        "https://ipv6.wtfismyip.com/text",
    ]
    .iter()
    .filter_map(|x| match x.parse::<Url>() {
        Ok(url) => Some(HttpSource {
            endpoint: url,
            decoder: Decoder::Plain,
        }),
        Err(err) => {
            debug!("Failed to parse endpoint for HTTP Source `{}`: {}", x, err);
            None
        }
    })
    .collect()
});

pub struct Http {
    client: Client,
}

impl Http {
    pub fn new() -> Result<Self> {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(2))
            .build()?;
        Ok(Self { client })
    }

    pub(crate) async fn get_ips(&self) -> Vec<IpAddr> {
        // Build a stream of futures
        let ip_stream = stream::iter(HTTP_SOURCES.iter().map(|src| {
            self.client
                .get(src.endpoint.clone())
                .send()
                .err_into()
                .and_then(move |response| {
                    src.decoder.decode(response).map_err(move |err| {
                        debug!("Failed to retrieve IP from `{}`: {}", src, err);
                        err
                    })
                })
        }));

        ip_stream
            .buffer_unordered(10)
            .filter_map(|result| async { result.ok() })
            .collect::<Vec<IpAddr>>()
            .await
    }
}

struct HttpSource {
    endpoint: Url,
    decoder: Decoder,
}

impl std::fmt::Display for HttpSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, ({})", self.endpoint, self.decoder)
    }
}

/// A decoder defines how to transform a source's reply to an IP address
/// * `Plain` expects the body to only contain the IP. It will attempt to strip any surrounding white space or quotes.
/// * `Json` expects the body to be a JSON object and will get the IP from the given field name.
#[derive(Debug)]
enum Decoder {
    Plain,
    Json(&'static str),
}

impl Decoder {
    pub async fn decode(&self, response: Response) -> Result<IpAddr> {
        let raw_ip = match self {
            Decoder::Plain => response.text().await?,
            Decoder::Json(field_name) => {
                let json_response = response.json::<serde_json::Value>().await?;
                let raw_ip = json_response
                    .get(field_name)
                    .ok_or(SourceError::JsonFieldMissing(field_name))?;
                raw_ip
                    .as_str()
                    .ok_or_else(|| SourceError::JsonFieldMalformed(field_name, raw_ip.to_string()))?
                    .to_string()
            }
        };
        raw_ip.parse().map_err(SourceError::RawIpMalformed)
    }
}

impl std::fmt::Display for Decoder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
