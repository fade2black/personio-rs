# personio-rs
![crates.io](https://img.shields.io/crates/v/personio-rs.svg)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

Async Rust client for [Personio](https://www.personio.com/) backed by [tokio](https://github.com/tokio-rs/tokio).

# Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
personio-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```
To get auth token:

```rust
use personio_rs::client::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Authenticating...");

    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;

    let client = Client::new("MY_COMPANY", "MY_APP")?;
    let creds = client.auth(&client_id, &client_secret).await?;

    println!("{creds}");

    Ok(())
}
```