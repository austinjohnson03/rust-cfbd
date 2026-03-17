use serde::Serialize;
use std::marker::PhantomData;

use crate::models::cfb::entity::division_classification::DivisionClassification;
use crate::models::cfb::entity::season_type::SeasonType;

pub struct NoQuery;
pub struct ById;
pub struct ByYear;

#[derive(Debug, Serialize)]
pub struct GetGameParams<Q> {
    #[serde(skip)]
    _query: PhantomData<Q>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    week: Option<u32>,

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

impl GetGameParams<NoQuery> {
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

    pub fn id(self, id: u32) -> GetGameParams<ById> {
        GetGameParams {
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

    pub fn year(self, year: u32) -> GetGameParams<ByYear> {
        GetGameParams {
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

impl<Q> GetGameParams<Q> {
    pub fn week(mut self, week: u32) -> Self {
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
    pub fn team(mut self, team: String) -> Self {
        self.team = Some(team);
        self
    }
    pub fn home(mut self, home: String) -> Self {
        self.home = Some(home);
        self
    }
    pub fn away(mut self, away: String) -> Self {
        self.away = Some(away);
        self
    }
    pub fn conference(mut self, conference: String) -> Self {
        self.conference = Some(conference);
        self
    }
}
