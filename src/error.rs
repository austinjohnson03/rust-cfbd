use thiserror::Error;

#[derive(Debug, Error)]
pub enum CFBDError {
    #[error("HTTP Error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Deserialize Error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("API Error {status}: {message}")]
    Api { status: u16, message: String },
}
