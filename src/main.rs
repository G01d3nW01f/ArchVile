mod structure;
use crate::structure::ArchVileArgs;
use clap::Parser;
use std::sync::Arc;
use tokio::fs::File;
use tokio::sync::Semaphore;
use tokio_util::io::ReaderStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. initialize struct and arg parse
    let args = Arc::new(ArchVileArgs::parse());
    let headers = Arc::new(args.parse_headers());
    let client = args.create_client()?;

    println!("--- ArchVile: Organized & Optimized ---");
    println!("Target:      {}", args.url);
    println!("Concurrency: {}", args.connections);
    println!("---------------------------------------");

    // 2. handle management for asynchronous execution
    let semaphore = Arc::new(Semaphore::new(args.connections));
    let mut handles = Vec::new();

    for i in 1..=args.connections {
        let _permit = Arc::clone(&semaphore).acquire_owned().await?;
        let args_clone = Arc::clone(&args);
        let headers_clone = Arc::clone(&headers);
        let client_clone = client.clone();

        let handle = tokio::spawn(async move {
            loop {
                let result = async {
                    // file stream
                    let file = File::open(&args_clone.file).await?;
                    let stream = ReaderStream::new(file);
                    let body = reqwest::Body::wrap_stream(stream);

                    // POST request
                    let res = client_clone
                        .post(&args_clone.url)
                        .headers((*headers_clone).clone())
                        .body(body)
                        .send()
                        .await?;
                    
                    Ok::<_, Box<dyn std::error::Error + Send + Sync>>(res.status())
                }.await;

                match result {
                    Ok(status) => println!("[Stream {:02}] Status: {}", i, status),
                    Err(e) => eprintln!("[Stream {:02}] Error: {}", i, e),
                }
            }
            #[allow(unreachable_code)]
            drop(_permit);
        });

        handles.push(handle);
    }

    // loop
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
