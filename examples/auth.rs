use personio_rs::client::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Authenticating...");

    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;

    let client = Client::new("PAIR_FINANCE", "TEST_APP")?;
    let creds = client.auth(&client_id, &client_secret).await?;

    println!("{creds}");

    Ok(())
}