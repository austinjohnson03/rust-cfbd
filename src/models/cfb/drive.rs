use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveClock {
    seconds: Option<i32>,
    minutes: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Drive {
    offense: String,
    offense_conference: Option<String>,
    defense: String,
    defense_conference: Option<String>,
    game_id: i32,
    id: String,
    drive_number: Option<i32>,
    scoring: bool,
    start_period: i32,
    start_yardline: i32,
    start_yards_to_goal: i32,
    start_time: DriveClock,
    end_period: i32,
    end_yardline: i32,
    end_yards_to_goal: i32,
    end_time: DriveClock,
    elapsed: DriveClock,
    plays: i32,
    yards: i32,
    drive_result: String,
    is_home_offense: bool,
    start_offense_score: i32,
    start_defense_score: i32,
    end_offense_score: i32,
    end_defense_score: i32,
}
