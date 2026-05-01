# ArchVile

ArchVile is a high-performance, asynchronous file upload stress tester written in Rust. It is designed to continuously stream a file to a target URL using multiple concurrent connections to test the robustness and throughput of web servers and API endpoints.

## Disclaimer

> [!WARNING]
> **This tool is developed for authorized security testing and educational purposes only.**
> 
> Using this software to attack targets without prior mutual consent is illegal. It is the end user's responsibility to obey all applicable local, state, and federal laws. The developers assume no liability and are not responsible for any misuse or damage caused by this program. Only use ArchVile on infrastructure you own or have explicit permission to test.

## Features

* **Asynchronous I/O**: Built on the `tokio` runtime for efficient resource management and high concurrency.
* **Streaming Uploads**: Utilizes `tokio-util` and `reqwest` to stream files directly from disk, minimizing memory overhead even with large files.
* **Customizable Concurrency**: Allows users to specify the number of simultaneous connections via a semaphore-controlled task pool.
* **Flexible Headers**: Supports passing multiple custom HTTP headers for authenticating or configuring requests.
* **SSL/TLS Flexibility**: Automatically ignores invalid certificates to facilitate testing in staging or development environments.

## Installation

Ensure you have the Rust toolchain installed (Edition 2024).

```bash
# Clone the repository
git clone <repository-url>
cd ArchVile

# Build the project
cargo build --release
```

## Usage

```bash
./target/release/ArchVile --url <TARGET_URL> --file <PATH_TO_FILE> [OPTIONS]
```

### Options

* `-u, --url <URL>`: The target URL for the POST requests.
* `-f, --file <FILE>`: Path to the local file to be uploaded.
* `-c, --connection <NUM>`: Number of concurrent upload streams (default: 1).
* `-H, --header "Name: Value"`: Add custom headers. This flag can be used multiple times.

### Example

Stream `data.bin` to a local server using 10 concurrent connections and a custom API key:

```bash
./target/release/ArchVile -u http://localhost:8080/upload -f ./data.bin -c 10 -H "X-API-Key: secret123"
```

## Project Structure

* `main.rs`: Manages the asynchronous runtime, task spawning, and the continuous request loop.
* `structure.rs`: Defines the command-line interface and helper methods for HTTP client and header configuration.
* `Cargo.toml`: Lists the project dependencies: `tokio`, `reqwest`, `clap`, and `tokio-util`.
```
