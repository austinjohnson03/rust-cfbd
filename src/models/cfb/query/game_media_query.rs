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
pub struct GameMediaQueryBuilder<Q> {
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

impl GameMediaQueryBuilder<NoQuery> {
    pub fn new() -> Self {
        GameMediaQueryBuilder {
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

    pub fn year(self, year: i32) -> GameMediaQueryBuilder<ByYear> {
        GameMediaQueryBuilder {
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

impl Default for GameMediaQueryBuilder<NoQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl GameMediaQueryBuilder<ByYear> {
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

impl<Q> GameMediaQueryBuilder<Q> {
    pub fn season_type(mut self, season_type: SeasonType) -> Self {
        self.season_type = Some(season_type);
        self
    }

    pub fn week(mut self, week: i32) -> Self {
        self.week = Some(week);
        self
    }

    pub fn team(mut self, team: impl Into<String>) -> Self {
        self.team = Some(team.into());
        self
    }

    pub fn conference(mut self, conference: impl Into<String>) -> Self {
        self.conference = Some(conference.into());
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
