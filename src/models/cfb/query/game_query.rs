use serde::Serialize;
use std::marker::PhantomData;

use crate::common::conversion::{IntoOptional, IntoOptionalString};
use crate::models::cfb::entity::division_classification::DivisionClassification;
use crate::models::cfb::entity::season_type::SeasonType;

pub struct NoQuery;
pub struct ById;
pub struct ByYear;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    week: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    season_type: Option<SeasonType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<DivisionClassification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    home: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    away: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conference: Option<String>,
}

pub struct GameQueryBuilder<Q> {
    _query: PhantomData<Q>,

    id: Option<i32>,
    year: Option<i32>,
    week: Option<i32>,
    season_type: Option<SeasonType>,
    classification: Option<DivisionClassification>,
    team: Option<String>,
    home: Option<String>,
    away: Option<String>,
    conference: Option<String>,
}

impl Default for GameQueryBuilder<NoQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl GameQueryBuilder<NoQuery> {
    pub fn new() -> Self {
        Self {
            _query: PhantomData,
            id: None,
            year: None,
            week: None,
            season_type: None,
            classification: None,
            team: None,
            home: None,
            away: None,
            conference: None,
        }
    }

    pub fn id(self, id: i32) -> GameQueryBuilder<ById> {
        GameQueryBuilder {
            _query: PhantomData,
            id: Some(id),
            year: None,
            week: self.week,
            season_type: self.season_type,
            classification: self.classification,
            team: self.team,
            home: self.home,
            away: self.away,
            conference: self.conference,
        }
    }

    pub fn year(self, year: i32) -> GameQueryBuilder<ByYear> {
        GameQueryBuilder {
            _query: PhantomData,
            id: None,
            year: Some(year),
            week: self.week,
            season_type: self.season_type,
            classification: self.classification,
            team: self.team,
            home: self.home,
            away: self.away,
            conference: self.conference,
        }
    }
}

impl<Q> GameQueryBuilder<Q> {
    pub fn week(mut self, week: impl IntoOptional<i32>) -> Self {
        self.week = week.into_optional();
        self
    }
    pub fn season_type(mut self, season_type: impl IntoOptional<SeasonType>) -> Self {
        self.season_type = season_type.into_optional();
        self
    }
    pub fn classification(
        mut self,
        classification: impl IntoOptional<DivisionClassification>,
    ) -> Self {
        self.classification = classification.into_optional();
        self
    }
    pub fn team(mut self, team: impl IntoOptionalString) -> Self {
        self.team = team.into_optional_string();
        self
    }
    pub fn home(mut self, home: impl IntoOptionalString) -> Self {
        self.home = home.into_optional_string();
        self
    }
    pub fn away(mut self, away: impl IntoOptionalString) -> Self {
        self.away = away.into_optional_string();
        self
    }
    pub fn conference(mut self, conference: impl IntoOptionalString) -> Self {
        self.conference = conference.into_optional_string();
        self
    }
}

impl GameQueryBuilder<ById> {
    pub fn build(self) -> GameQuery {
        GameQuery {
            id: self.id,
            year: None,
            week: None,
            season_type: None,
            classification: None,
            team: None,
            home: None,
            away: None,
            conference: None,
        }
    }
}

impl GameQueryBuilder<ByYear> {
    pub fn build(self) -> GameQuery {
        GameQuery {
            id: None,
            year: self.year,
            week: self.week,
            season_type: self.season_type,
            classification: self.classification,
            team: self.team,
            home: self.home,
            away: self.away,
            conference: self.conference,
        }
    }
}
