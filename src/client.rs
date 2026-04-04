use crate::config::Config;
use crate::error::CFBDError;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::time::{Duration, sleep};

#[derive(Clone)]
pub struct CfbdClient {
    http: Client,
    config: Config,
}

impl CfbdClient {
    pub fn new(config: Config) -> Self {
        Self {
            http: Client::new(),
            config,
        }
    }

    pub async fn get<T: DeserializeOwned, P: Serialize>(
        &self,
        path: &str,
        params: Option<&P>,
    ) -> Result<T, CFBDError> {
        if let Some(ms) = self.config.rate_limit_ms {
            sleep(Duration::from_millis(ms)).await;
        }
        let url = format!("{}/{}", self.config.host, path);

        let resp = match params {
            Some(p) => {
                self.http
                    .get(&url)
                    .bearer_auth(&self.config.api_key)
                    .query(&p)
                    .send()
                    .await?
            }
            None => {
                self.http
                    .get(&url)
                    .bearer_auth(&self.config.api_key)
                    .send()
                    .await?
            }
        };

        match resp.status().as_u16() {
            200..=299 => {
                let body = resp.text().await?;
                serde_json::from_str::<T>(&body).map_err(|e| CFBDError::Deserialize(e, body))
            }
            400 => Err(CFBDError::BadRequest {
                message: resp.text().await.unwrap_or_default(),
            }),
            401 => Err(CFBDError::Unauthorized),
            404 => Err(CFBDError::NotFound { resource: url }),
            429 => {
                let retry_after = resp
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(60);

                Err(CFBDError::RateLimited { retry_after })
            }
            status @ 500..=599 => Err(CFBDError::ServerError {
                status,
                message: resp.text().await.unwrap_or_default(),
            }),
            status => Err(CFBDError::Unexpected {
                status,
                message: resp.text().await.unwrap_or_default(),
            }),
        }
    }

    pub async fn get_raw_json<P: Serialize>(
        &self,
        path: &str,
        params: &P,
    ) -> Result<String, CFBDError> {
        if let Some(ms) = self.config.rate_limit_ms {
            sleep(Duration::from_millis(ms)).await;
        }
        let url = format!("{}/{}", self.config.host, path);

        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.config.api_key)
            .query(params)
            .send()
            .await?;
        Ok(resp.text().await?)
    }
}
