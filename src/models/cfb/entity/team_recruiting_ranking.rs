use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamRecruitingRanking {
    year: i32,
    rank: i32,
    team: String,
    points: f64,
}
