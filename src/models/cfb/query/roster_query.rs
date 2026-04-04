use serde::Serialize;

use crate::common::conversion::IntoOptionalString;
use crate::models::cfb::entity::division_classification::DivisionClassification;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterQuery {
    pub team: Option<String>,
    pub year: Option<i32>,
    pub classification: Option<DivisionClassification>,
}

pub struct RosterQueryBuilder {
    pub team: Option<String>,
    pub year: Option<i32>,
    pub classification: Option<DivisionClassification>,
}

impl Default for RosterQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RosterQueryBuilder {
    pub fn new() -> Self {
        RosterQueryBuilder {
            team: None,
            year: None,
            classification: None,
        }
    }

    pub fn team(mut self, team: impl IntoOptionalString) -> Self {
        self.team = team.into_optional_string();
        self
    }

    pub fn year(mut self, year: impl Into<Option<i32>>) -> Self {
        self.year = year.into();
        self
    }

    pub fn classification(
        mut self,
        classification: impl Into<Option<DivisionClassification>>,
    ) -> Self {
        self.classification = classification.into();
        self
    }

    pub fn build(self) -> RosterQuery {
        RosterQuery {
            team: self.team,
            year: self.year,
            classification: self.classification,
        }
    }
}
