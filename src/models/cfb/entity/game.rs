use crate::models::cfb::entity::division_classification::DivisionClassification;
use crate::models::cfb::entity::season_type::SeasonType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: i32,
    pub season: i32,
    pub week: i32,
    pub season_type: SeasonType,
    pub start_date: DateTime<Utc>,
    pub start_time_tbd: Option<bool>,
    pub completed: bool,
    pub neutral_site: bool,
    pub conference_game: bool,
    pub attendance: Option<i32>,
    pub venue_id: Option<i32>,
    pub venue: Option<String>,
    pub home_id: i32,
    pub home_team: String,
    pub home_conference: Option<String>,
    pub home_classification: Option<DivisionClassification>,
    pub home_points: Option<i32>,
    pub home_line_scores: Option<Vec<i32>>,
    pub home_postgame_win_probability: Option<f64>,
    pub home_pregame_elo: Option<i32>,
    pub home_postgame_elo: Option<i32>,
    pub away_id: i32,
    pub away_team: String,
    pub away_conference: Option<String>,
    pub away_classification: Option<DivisionClassification>,
    pub away_points: Option<i32>,
    pub away_line_scores: Option<Vec<i32>>,
    pub away_postgame_win_probability: Option<f64>,
    pub away_pregame_elo: Option<i32>,
    pub away_postgame_elo: Option<i32>,
    pub excitement_index: Option<f64>,
    pub highlights: Option<String>,
    pub notes: Option<String>,
}
