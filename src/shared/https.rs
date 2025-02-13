use crate::Result;
use reqwest::{header, Body, Client};
use std::time::Duration;

// #[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(partner_id: &str, app_id: &str) -> Result<HttpClient> {
        let mut headers = header::HeaderMap::new();
        let partner_id = header::HeaderValue::from_str(partner_id)?;
        let app_id = header::HeaderValue::from_str(app_id)?;
        let content_type = header::HeaderValue::from_str("application/json")?;
        let accept = header::HeaderValue::from_str("application/json")?;

        headers.insert("X-Personio-Partner-ID", partner_id);
        headers.insert("X-Personio-App-ID", app_id);
        headers.insert("Content-Type", content_type);
        headers.insert("Accept", accept);

        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .default_headers(headers)
            .build()?;

        Ok(HttpClient { client })
    }

    // pub async fn get(&self, url: &str) -> Result<String> {
    //     let response = self.client.get(url).send().await?;
    //     let body = response.text().await?;
    //     Ok(body)
    // }

    pub async fn post<T: Into<Body>>(&self, url: &str, body: T) -> Result<String> {
        let response = self.client.post(url).body(body).send().await?;
        let body = response.text().await?;
        Ok(body)
    }
}
