use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::multipart;
use std::sync::Arc;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use super::definition::ArchVileArgs;

impl ArchVileArgs {
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
                let file_size = metadata.len();

                let file_name = self.file.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("upload.file")
                    .to_string();

                let stream = ReaderStream::new(file);
                let body = reqwest::Body::wrap_stream(stream);

                let part = multipart::Part::stream_with_length(body, file_size)
                    .file_name(file_name);
                
                let form = multipart::Form::new()
                    .part(self.field.clone(), part);

                let mut final_headers = (*headers).clone();
                final_headers.remove(CONTENT_TYPE);

                let res = client
                    .post(&self.url)
                    .headers(final_headers)
                    .multipart(form)
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
