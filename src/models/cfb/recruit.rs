use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum RecruitClassification {
    #[serde(rename = "JUCO")]
    Juco,
    #[serde(rename = "PrepSchool")]
    PrepSchool,
    #[serde(rename = "HighSchool")]
    HighSchool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HometownInfo {
    pub fips_code: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recruit {
    pub id: String,
    pub athelte_id: Option<String>,
    pub recruit_type: RecruitClassification,
    pub year: i32,
    pub ranking: Option<i32>,
    pub name: String,
    pub school: Option<String>,
    pub committed_to: Option<String>,
    pub position: Option<String>,
    pub height: Option<f64>,
    pub weight: Option<i32>,
    pub stars: i32,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub country: Option<String>,
    pub hometown_info: HometownInfo,
}
