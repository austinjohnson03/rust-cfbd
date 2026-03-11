use serde::Serialize;

use crate::models::cfb::division_classification::DivisionClassification;

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

impl RosterQueryBuilder {
    pub fn new() -> Self {
        RosterQueryBuilder {
            team: None,
            year: None,
            classification: None,
        }
    }

    pub fn team(mut self, team: impl Into<String>) -> Self {
        self.team = Some(team.into());
        self
    }

    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    pub fn classification(mut self, classification: DivisionClassification) -> Self {
        self.classification = Some(classification);
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
