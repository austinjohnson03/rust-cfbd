use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayClock {
    seconds: Option<i32>,
    minutes: Option<i32>,
}
