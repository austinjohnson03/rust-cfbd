#[derive(Clone)]
pub struct Config {
    pub api_key: String,
    pub host: String,
    pub rate_limit_ms: Option<u64>,
}

impl Config {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            host: "https://apinext.collegefootballdata.com".to_string(),
            rate_limit_ms: Some(2000),
        }
    }

    pub fn with_host_url(mut self, new_url: String) -> Self {
        self.host = new_url;
        self
    }

    pub fn no_rate_limit(mut self) -> Self {
        self.rate_limit_ms = None;
        self
    }

    pub fn rate_limit_ms(mut self, ms: u64) -> Self {
        self.rate_limit_ms = Some(ms);
        self
    }
}
