pub struct Config {
    pub api_key: String,
    pub host: String,
}

impl Config {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            host: "https://apinext.collegefootballdata.com".to_string(),
        }
    }

    pub fn with_host_url(mut self, new_url: String) -> Self {
        self.host = new_url;
        self
    }
}
