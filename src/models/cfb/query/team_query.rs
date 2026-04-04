use crate::{
    common::conversion::IntoOptionalString,
    models::cfb::entity::division_classification::DivisionClassification,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<DivisionClassification>,
}

pub struct TeamQueryBuilder {
    conference: Option<String>,
    year: Option<i32>,
    classification: Option<DivisionClassification>,
}

impl Default for TeamQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TeamQueryBuilder {
    pub fn new() -> Self {
        TeamQueryBuilder {
            conference: None,
            year: None,
            classification: None,
        }
    }

    pub fn conference(mut self, conference: impl IntoOptionalString) -> Self {
        self.conference = conference.into_optional_string();
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

    pub fn build(self) -> TeamQuery {
        TeamQuery {
            conference: self.conference,
            year: self.year,
            classification: self.classification,
        }
    }
}
