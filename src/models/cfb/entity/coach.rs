use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoachSeason {
    pub school: String,
    pub year: i32,
    pub games: i32,
    pub wins: i32,
    pub losses: i32,
    pub ties: i32,
    pub preseason_rank: Option<i32>,
    pub postseason_rank: Option<i32>,
    pub srs: Option<f64>,
    pub sp_overall: Option<f64>,
    pub sp_offense: Option<f64>,
    pub sp_defense: Option<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Coach {
    pub first_name: String,
    pub last_name: String,
    pub hire_date: Option<DateTime<Utc>>,
    pub seasons: Vec<CoachSeason>,
    #[cfg(test)]
    pub simulate_error: Option<String>,
}
