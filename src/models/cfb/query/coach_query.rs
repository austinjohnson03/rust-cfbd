use crate::common::conversion::{IntoOptional, IntoOptionalString};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoachQuery {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub team: Option<String>,
    pub year: Option<i32>,
    pub min_year: Option<i32>,
    pub max_year: Option<i32>,
}

pub struct CoachQueryBuilder {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub team: Option<String>,
    pub year: Option<i32>,
    pub min_year: Option<i32>,
    pub max_year: Option<i32>,
}

impl CoachQueryBuilder {
    pub fn new() -> Self {
        CoachQueryBuilder {
            first_name: None,
            last_name: None,
            team: None,
            year: None,
            min_year: None,
            max_year: None,
        }
    }

    pub fn first_name(mut self, first_name: impl IntoOptionalString) -> Self {
        self.first_name = first_name.into_optional_string();
        self
    }

    pub fn last_name(mut self, last_name: impl IntoOptionalString) -> Self {
        self.last_name = last_name.into_optional_string();
        self
    }

    pub fn team(mut self, team: impl IntoOptionalString) -> Self {
        self.team = team.into_optional_string();
        self
    }

    pub fn year(mut self, year: impl IntoOptional<i32>) -> Self {
        self.year = year.into_optional();
        self
    }

    pub fn min_year(mut self, min_year: impl IntoOptional<i32>) -> Self {
        self.min_year = min_year.into_optional();
        self
    }

    pub fn max_year(mut self, max_year: impl IntoOptional<i32>) -> Self {
        self.max_year = max_year.into_optional();
        self
    }

    pub fn build(self) -> CoachQuery {
        CoachQuery {
            first_name: self.first_name,
            last_name: self.last_name,
            team: self.team,
            year: self.year,
            min_year: self.min_year,
            max_year: self.max_year,
        }
    }
}

impl Default for CoachQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
