use std::fmt::format;

use crate::config::Config;
use crate::error::CFBDError;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;

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

    pub(crate) async fn get<T: DeserializeOwned, P: Serialize>(
        &self,
        path: &str,
        params: &P,
    ) -> Result<T, CFBDError> {
        let url = format!("{}/{}", self.config.host, path);

        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.config.api_key)
            .query(params)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(CFBDError::Api {
                status: resp.status().as_u16(),
                message: resp.text().await?,
            });
        }

        let data = resp.json::<T>().await?;
        Ok(data)
    }

    pub async fn get_raw_json(&self, path: &str) -> Result<String, CFBDError> {
        let url = format!("{}/{}", self.config.host, path);

        let resp = self
            .http
            .get(&url)
            .bearer_auth(&self.config.api_key)
            .send()
            .await?;
        Ok(resp.text().await?)
    }
}
