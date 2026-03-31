use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum StatValue {
    Int(i32),
    Text(String),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamStat {
    pub season: i32,
    pub team: String,
    pub conference: String,
    pub stat_name: String,
    pub stat_value: StatValue,
}
