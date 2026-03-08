use serde::{Deserialize, Serialize};

use crate::models::cfb::division_classification::DivisionClassification;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Conference {
    pub id: u32,
    pub name: String,
    pub short_name: Option<String>,
    pub abbreviation: Option<String>,
    pub classification: Option<DivisionClassification>,
}
