use crate::models::cfb::media_type::MediaType;
use crate::models::cfb::season_type::SeasonType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct GameMedia {
    pub id: i32,
    pub season: i32,
    pub week: i32,
    pub season_type: SeasonType,
    pub start_time: Option<DateTime<Utc>>,
    pub is_start_time_tbd: Option<bool>,
    pub home_team: String,
    pub home_conference: Option<String>,
    pub away_team: String,
    pub away_conference: Option<String>,
    pub media_type: MediaType,
    pub outlet: Option<String>,
}
