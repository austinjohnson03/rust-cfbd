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

        if !resp.status().is_success() {
            return Err(CFBDError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await?,
            });
        }

        let data = resp.json::<T>().await?;
        Ok(data)
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
