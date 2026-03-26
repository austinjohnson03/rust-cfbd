use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub patron_level: f32,
    pub remaining_calls: f32,
}

impl UserInfo {
    pub fn new(patron_level: f32, remaining_calls: f32) -> Self {
        Self {
            patron_level,
            remaining_calls,
        }
    }
}

impl fmt::Display for UserInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "UserInfo: Patron Level: {} Remaining Calls: {}",
            self.patron_level, self.remaining_calls
        )
    }
}
