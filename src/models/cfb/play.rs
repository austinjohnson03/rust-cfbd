use crate::models::cfb::play_clock::PlayClock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Play {
    id: String,
    drive_id: String,
    game_id: i32,
    drive_number: Option<i32>,
    play_number: Option<i32>,
    offense: String,
    offense_confernce: Option<String>,
    offense_score: i32,
    defense: String,
    home: String,
    away: String,
    defense_conference: Option<String>,
    defense_score: i32,
    period: i32,
    clock: PlayClock,
    offense_timeouts: Option<i32>,
    defense_timeouts: Option<i32>,
    yardline: i32,
    yards_to_goal: i32,
    down: i32,
    distance: i32,
    yards_gained: i32,
    scoring: bool,
    play_type: String,
    play_text: Option<String>,
    ppa: Option<f64>,
    wallclock: Option<String>,
}
