use serde::Serialize;
use std::marker::PhantomData;

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
    pub fn week(mut self, week: i32) -> Self {
        self.week = Some(week);
        self
    }
    pub fn season_type(mut self, season_type: SeasonType) -> Self {
        self.season_type = Some(season_type);
        self
    }
    pub fn classification(mut self, classification: DivisionClassification) -> Self {
        self.classification = Some(classification);
        self
    }
    pub fn team(mut self, team: impl Into<String>) -> Self {
        self.team = Some(team.into());
        self
    }
    pub fn home(mut self, home: impl Into<String>) -> Self {
        self.home = Some(home.into());
        self
    }
    pub fn away(mut self, away: impl Into<String>) -> Self {
        self.away = Some(away.into());
        self
    }
    pub fn conference(mut self, conference: impl Into<String>) -> Self {
        self.conference = Some(conference.into());
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
