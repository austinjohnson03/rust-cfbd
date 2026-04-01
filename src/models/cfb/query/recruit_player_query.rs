use serde::Serialize;
use std::marker::PhantomData;

use crate::models::cfb::entity::recruit::RecruitClassification;

pub struct InvalidQuery;
pub struct ValidQuery;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecruitPlayerQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<RecruitClassification>,
}

pub struct RecruitPlayerQueryBuilder<Q> {
    _query: PhantomData<Q>,
    year: Option<i32>,
    team: Option<String>,
    position: Option<String>,
    state: Option<String>,
    classification: Option<RecruitClassification>,
}

impl RecruitPlayerQueryBuilder<InvalidQuery> {
    pub fn new() -> Self {
        RecruitPlayerQueryBuilder {
            _query: PhantomData,
            year: None,
            team: None,
            position: None,
            state: None,
            classification: None,
        }
    }
    pub fn year(self, year: i32) -> RecruitPlayerQueryBuilder<ValidQuery> {
        RecruitPlayerQueryBuilder {
            _query: PhantomData,
            year: Some(year),
            team: self.team,
            position: self.position,
            state: self.state,
            classification: self.classification,
        }
    }

    pub fn team(self, team: impl Into<String>) -> RecruitPlayerQueryBuilder<ValidQuery> {
        RecruitPlayerQueryBuilder {
            _query: PhantomData,
            year: self.year,
            team: Some(team.into()),
            position: self.position,
            state: self.state,
            classification: self.classification,
        }
    }
}

impl Default for RecruitPlayerQueryBuilder<InvalidQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Q> RecruitPlayerQueryBuilder<Q> {
    pub fn position(mut self, position: impl Into<String>) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    pub fn classification(mut self, classification: RecruitClassification) -> Self {
        self.classification = Some(classification);
        self
    }
}

impl RecruitPlayerQueryBuilder<ValidQuery> {
    pub fn build(self) -> RecruitPlayerQuery {
        RecruitPlayerQuery {
            year: self.year,
            team: self.team,
            position: self.position,
            state: self.state,
            classification: self.classification,
        }
    }
}
