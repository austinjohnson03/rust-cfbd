use thiserror::Error;

#[derive(Debug, Error)]
pub enum CFBDError {
    #[error("Network error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Deserialize error: {0}\nBody: {1}")]
    Deserialize(serde_json::Error, String),

    // 400
    #[error("Bad request: {message}")]
    BadRequest { message: String },

    // 401
    #[error("Unauthorized: invalid or missing API key")]
    Unauthorized,

    // 404
    #[error("Not found: {resource}")]
    NotFound { resource: String },

    // 429
    #[error("Rate limit hit. Please try again after {retry_after}s")]
    RateLimited { retry_after: u64 },

    // 500-599
    #[error("Server error({status}): {message}")]
    ServerError { status: u16, message: String },

    #[error("Unexpected HTTP Error ({status}): {message}")]
    Unexpected { status: u16, message: String },
}
