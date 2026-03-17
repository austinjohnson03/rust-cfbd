use crate::models::cfb::entity::division_classification::DivisionClassification;
use crate::models::cfb::entity::media_type::MediaType;
use crate::models::cfb::entity::season_type::SeasonType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct NoQuery;
pub struct ByYear;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameMediaQuery {
    pub year: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season_type: Option<SeasonType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<DivisionClassification>,
}

#[derive(Debug, Serialize)]
pub struct GameMediaParams<Q> {
    #[serde(skip)]
    _query: PhantomData<Q>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season_type: Option<SeasonType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<DivisionClassification>,
}

impl GameMediaParams<NoQuery> {
    pub fn new() -> Self {
        GameMediaParams {
            _query: PhantomData,
            year: None,
            season_type: None,
            week: None,
            team: None,
            conference: None,
            media_type: None,
            classification: None,
        }
    }

    pub fn year(self, year: i32) -> GameMediaParams<ByYear> {
        GameMediaParams {
            _query: PhantomData,
            year: Some(year),
            season_type: self.season_type,
            week: self.week,
            team: self.team,
            conference: self.conference,
            media_type: self.media_type,
            classification: self.classification,
        }
    }
}

impl Default for GameMediaParams<NoQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl GameMediaParams<ByYear> {
    pub fn build(self) -> GameMediaQuery {
        GameMediaQuery {
            year: self.year.unwrap(),
            season_type: self.season_type,
            week: self.week,
            team: self.team,
            conference: self.conference,
            media_type: self.media_type,
            classification: self.classification,
        }
    }
}

impl<Q> GameMediaParams<Q> {
    pub fn season_type(mut self, season_type: SeasonType) -> Self {
        self.season_type = Some(season_type);
        self
    }

    pub fn week(mut self, week: i32) -> Self {
        self.week = Some(week);
        self
    }

    pub fn team(mut self, team: String) -> Self {
        self.team = Some(team);
        self
    }

    pub fn conference(mut self, conference: String) -> Self {
        self.conference = Some(conference);
        self
    }

    pub fn media_type(mut self, media_type: MediaType) -> Self {
        self.media_type = Some(media_type);
        self
    }

    pub fn classification(mut self, classification: DivisionClassification) -> Self {
        self.classification = Some(classification);
        self
    }
}
