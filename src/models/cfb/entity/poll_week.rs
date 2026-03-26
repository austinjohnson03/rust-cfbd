use crate::models::cfb::entity::season_type::SeasonType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PollRank {
    pub rank: Option<i32>,
    pub team_id: i32,
    pub school: String,
    pub confernce: Option<String>,
    pub first_place_votes: Option<i32>,
    pub points: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    pub poll: String,
    pub ranks: Vec<PollRank>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PollWeek {
    pub season: i32,
    pub season_type: SeasonType,
    pub week: i32,
    pub polls: Vec<Poll>,
}
