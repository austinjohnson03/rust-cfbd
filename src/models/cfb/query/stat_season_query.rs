use serde::Serialize;
use std::marker::PhantomData;

pub struct InvalidQuery;
pub struct ValidQuery;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatSeasonQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_week: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_week: Option<i32>,
}

pub struct StatSeasonQueryBuilder<Q> {
    _query: PhantomData<Q>,
    year: Option<i32>,
    team: Option<String>,
    conference: Option<String>,
    start_week: Option<i32>,
    end_week: Option<i32>,
}

impl StatSeasonQueryBuilder<InvalidQuery> {
    pub fn new() -> Self {
        StatSeasonQueryBuilder {
            _query: PhantomData,
            year: None,
            team: None,
            conference: None,
            start_week: None,
            end_week: None,
        }
    }
    pub fn year(self, year: i32) -> StatSeasonQueryBuilder<ValidQuery> {
        StatSeasonQueryBuilder {
            _query: PhantomData,
            year: Some(year),
            team: self.team,
            conference: self.conference,
            start_week: self.start_week,
            end_week: self.end_week,
        }
    }

    pub fn team(self, team: impl Into<String>) -> StatSeasonQueryBuilder<ValidQuery> {
        StatSeasonQueryBuilder {
            _query: PhantomData,
            year: self.year,
            team: Some(team.into()),
            conference: self.conference,
            start_week: self.start_week,
            end_week: self.end_week,
        }
    }
}

impl Default for StatSeasonQueryBuilder<InvalidQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Q> StatSeasonQueryBuilder<Q> {
    pub fn conference(mut self, conference: impl Into<String>) -> Self {
        self.conference = Some(conference.into());
        self
    }
    pub fn start_week(mut self, start_week: i32) -> Self {
        self.start_week = Some(start_week);
        self
    }
    pub fn end_week(mut self, end_week: i32) -> Self {
        self.end_week = Some(end_week);
        self
    }
}

impl StatSeasonQueryBuilder<ValidQuery> {
    pub fn build(self) -> StatSeasonQuery {
        StatSeasonQuery {
            year: self.year,
            team: self.team,
            conference: self.conference,
            start_week: self.start_week,
            end_week: self.end_week,
        }
    }
}
