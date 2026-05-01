use clap::Parser;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::path::PathBuf;
use std::str::FromStr;
//use std::sync::Arc;

#[derive(Parser, Debug, Clone)]
#[command(name = "ArchVile", about = "Continuous file upload stresser", version = "1.0.0")]
pub struct ArchVileArgs {
    /// Target URL
    #[arg(short = 'u', long = "url")]
    pub url: String,

    /// Custom header in "Name: Value" format (Multiple allowed)
    #[arg(short = 'H', long = "header", action = clap::ArgAction::Append)]
    pub headers: Vec<String>,

    /// Path to the file to upload
    #[arg(short = 'f', long = "file")]
    pub file: PathBuf,

    /// Number of concurrent connections
    #[arg(short = 'c', long = "connection", default_value_t = 1)]
    pub connections: usize,
}

impl ArchVileArgs {
    /// HeaderMap generate
    pub fn parse_headers(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        for h in &self.headers {
            let parts: Vec<&str> = h.splitn(2, ':').collect();
            if parts.len() == 2 {
                if let (Ok(key), Ok(value)) = (
                    HeaderName::from_str(parts[0].trim()),
                    HeaderValue::from_str(parts[1].trim()),
                ) {
                    header_map.insert(key, value);
                } else {
                    eprintln!("Warning: Invalid header format in '{}'", h);
                }
            }
        }
        header_map
    }

    /// HTTP client（ignore SSL issue)
    pub fn create_client(&self) -> Result<reqwest::Client, reqwest::Error> {
        reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
    }
}
