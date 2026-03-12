use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStat {
    season: i32,
    player_id: String,
    player: String,
    position: String,
    team: String,
    conference: String,
    category: String,
    stat_type: String,
    stat: String,
}
