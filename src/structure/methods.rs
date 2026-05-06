use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;
use super::definition::ArchVileArgs;

impl ArchVileArgs {
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
                }
            }
        }
        header_map
    }
}
