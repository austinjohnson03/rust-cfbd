use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    patron_level: f32,
    remaining_calls: f32,
}

impl UserInfo {
    pub fn new(patron_level: f32, remaining_calls: f32) -> Self {
        Self {
            patron_level,
            remaining_calls,
        }
    }
}
