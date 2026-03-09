use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecruitType {
    HighSchool,
    #[serde(rename = "JUCO")]
    Juco,
    PrepSchool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HometownInfo {
    pub fips_code: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recruit {
    pub id: String,
    pub athlete_id: String,
    pub recruit_type: RecruitType,
    pub year: u32,
    pub ranking: Option<u32>,
    pub name: String,
    pub school: Option<String>,
    pub committed_to: Option<String>,
    pub position: Option<String>, // Change to enum once I get all options
    pub height: Option<f64>,
    pub weight: Option<u32>,
    pub stars: u32,
    pub rating: f64,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub country: Option<String>,
    pub hometown_info: Option<HometownInfo>,
}
