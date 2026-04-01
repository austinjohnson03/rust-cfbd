use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RosterPlayer {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub team: String,
    pub height: Option<f64>,
    pub weight: Option<i32>,
    pub jersey: Option<i32>,
    pub position: Option<String>,
    pub home_city: Option<String>,
    pub home_state: Option<String>,
    pub home_country: Option<String>,
    pub home_latitude: Option<f64>,
    pub home_longitude: Option<f64>,
    #[serde(rename = "homeCountyFIPS")]
    pub home_county_fips: Option<String>,
    pub recruit_ids: Option<Vec<String>>,
}
