use super::auth::Auth;
use crate::shared::https::HttpClient;
use crate::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Client {
    inner: Arc<Mutex<HttpClient>>,
}

impl Client {
    pub fn new(partner_id: &str, app_id: &str) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(HttpClient::new(partner_id, app_id)?)),
        })
    }

    pub async fn auth(&self, client_id: &str, client_secret: &str) -> Result<String> {
        let auth_client = Auth::new(self.inner.clone());
        let res = auth_client.token(client_id, client_secret).await?;

        Ok(res)
    }
}
