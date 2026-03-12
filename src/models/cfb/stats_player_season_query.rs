use serde::Serialize;
use std::marker::PhantomData;

use crate::models::cfb::season_type::SeasonType;

pub struct InvalidQuery;
pub struct ValidQuery;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsPlayerSeasonQuery {
    pub year: i32,
    pub conference: Option<String>,
    pub team: Option<String>,
    pub start_week: Option<i32>,
    pub end_week: Option<i32>,
    pub season_type: Option<SeasonType>,
    pub category: Option<String>,
}

pub struct StatsPlayerSeasonQueryBuilder<Q> {
    _query: PhantomData<Q>,
    year: Option<i32>,
    conference: Option<String>,
    team: Option<String>,
    start_week: Option<i32>,
    end_week: Option<i32>,
    season_type: Option<SeasonType>,
    category: Option<String>,
}

impl StatsPlayerSeasonQueryBuilder<InvalidQuery> {
    pub fn new() -> Self {
        StatsPlayerSeasonQueryBuilder {
            _query: PhantomData,
            year: None,
            conference: None,
            team: None,
            start_week: None,
            end_week: None,
            season_type: None,
            category: None,
        }
    }

    pub fn year(self, year: i32) -> StatsPlayerSeasonQueryBuilder<ValidQuery> {
        StatsPlayerSeasonQueryBuilder {
            _query: PhantomData,
            year: Some(year),
            conference: self.conference,
            team: self.team,
            start_week: self.start_week,
            end_week: self.end_week,
            season_type: self.season_type,
            category: self.category,
        }
    }
}

impl Default for StatsPlayerSeasonQueryBuilder<InvalidQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsPlayerSeasonQueryBuilder<ValidQuery> {
    pub fn build(self) -> StatsPlayerSeasonQuery {
        StatsPlayerSeasonQuery {
            year: self.year.unwrap(),
            conference: self.conference,
            team: self.team,
            start_week: self.start_week,
            end_week: self.end_week,
            season_type: self.season_type,
            category: self.category,
        }
    }
}

impl<Q> StatsPlayerSeasonQueryBuilder<Q> {
    pub fn conference(mut self, conference: impl Into<String>) -> Self {
        self.conference = Some(conference.into());
        self
    }

    pub fn team(mut self, team: impl Into<String>) -> Self {
        self.team = Some(team.into());
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
    pub fn season_type(mut self, season_type: SeasonType) -> Self {
        self.season_type = Some(season_type);
        self
    }
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
}
