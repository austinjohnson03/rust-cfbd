use crate::models::cfb::entity::division_classification::DivisionClassification;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamLocation {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<String>,
    pub capacity: Option<i32>,
    pub construction_year: Option<i32>,
    pub grass: Option<bool>,
    pub dome: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: i32,
    pub school: String,
    pub mascot: Option<String>,
    pub abbreviation: Option<String>,
    pub alternate_names: Option<Vec<String>>,
    pub conference: Option<String>,
    pub division: Option<String>,
    pub classification: Option<DivisionClassification>,
    pub color: Option<String>,
    pub alternate_color: Option<String>,
    pub logos: Option<Vec<String>>,
    pub twitter: Option<String>,
    pub location: Option<TeamLocation>,
}
