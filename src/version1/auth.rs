use crate::shared::https::HttpClient;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) struct Auth {
    inner: Arc<Mutex<HttpClient>>,
}

#[derive(Serialize, Deserialize)]
struct AuthRequestBody<'a> {
    client_id: &'a str,
    client_secret: &'a str,
}

impl Auth {
    pub(crate) fn new(client: Arc<Mutex<HttpClient>>) -> Self {
        Self { inner: client }
    }

    pub(crate) async fn token(&self, client_id: &str, client_secret: &str) -> Result<String> {
        let url = "https://api.personio.de/v1/auth";

        let body = AuthRequestBody {
            client_id,
            client_secret,
        };
        let body = serde_json::to_string(&body)?;
        let result = self.inner.lock().await.post(url, body).await?;

        Ok(result)
    }
}
