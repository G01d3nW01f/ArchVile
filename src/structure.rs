use clap::Parser;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use reqwest::multipart;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[derive(Parser, Debug, Clone)]
pub struct ArchVileArgs {
    #[arg(short = 'u', long = "url")]
    pub url: String,

    #[arg(short = 'H', long = "header", action = clap::ArgAction::Append)]
    pub headers: Vec<String>,

    #[arg(short = 'f', long = "file")]
    pub file: PathBuf,

    #[arg(short = 'F', long = "field", default_value = "file")]
    pub field: String,

    #[arg(short = 'c', long = "connection", default_value_t = 1)]
    pub connections: usize,
}

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

    pub fn create_client(&self) -> Result<reqwest::Client, reqwest::Error> {
        reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
    }

    pub async fn execute_upload_loop(
        self: Arc<Self>,
        id: usize,
        client: reqwest::Client,
        headers: Arc<HeaderMap>,
    ) {
        println!("[Stream {:02}] Ritual started (Field: '{}')", id, self.field);

        loop {
            let result = async {
                let file = File::open(&self.file).await?;
                let metadata = file.metadata().await?;
                let file_size = metadata.len(); // file size

                let file_name = self.file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("upload.bin")
                    .to_string();

                let stream = ReaderStream::new(file);
                let body = reqwest::Body::wrap_stream(stream);

                // create Parts declare size explicitly for avoidance to clear with parse   
                let part = multipart::Part::stream_with_length(body, file_size)
                    .file_name(file_name);
                
                let form = multipart::Form::new()
                    .part(self.field.clone(), part);

                // avoid the "Content-Type" [Crucial]
                // save multipart boundary
                let mut final_headers = (*headers).clone();
                final_headers.remove(CONTENT_TYPE);

                let res = client
                    .post(&self.url)
                    .headers(final_headers) // set specific user header
                    .multipart(form)        // auto set Content-Type compatible
                    .send()
                    .await?;

                Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res.status())
            }
            .await;

            match result {
                Ok(status) => println!("[Stream {:02}] Status: {}", id, status),
                Err(e) => {
                    eprintln!("[Stream {:02}] Error: {}", id, e);
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
            }
        }
    }
}
