mod structure;

use crate::structure::ArchVileArgs;
use clap::Parser;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arc::new(ArchVileArgs::parse());
    let headers = Arc::new(args.parse_headers());
    let client = args.create_client()?;

    println!("--- ArchVile: Multipart Siege Mode ---");
    println!("Target URL:  {}", args.url);
    println!("File Field:  {}", args.field); 
    println!("Concurrency: {}", args.connections);
    println!("---------------------------------------");

    let semaphore = Arc::new(Semaphore::new(args.connections));
    let mut handles = Vec::new();

    for i in 1..=args.connections {
        let args_clone = Arc::clone(&args);
        let headers_clone = Arc::clone(&headers);
        let client_clone = client.clone();
        let sem_clone = Arc::clone(&semaphore);

        let handle = tokio::spawn(async move {
            let _permit = sem_clone.acquire_owned().await.expect("Semaphore fail");
            args_clone.execute_upload_loop(i, client_clone, headers_clone).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
