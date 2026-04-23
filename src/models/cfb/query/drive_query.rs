use crate::common::conversion::{IntoOptional, IntoOptionalString};
use crate::models::cfb::entity::division_classification::DivisionClassification;
use crate::models::cfb::entity::season_type::SeasonType;
use serde::Serialize;
use std::marker::PhantomData;

pub struct NoQuery;
pub struct ByYear;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveQuery {
    year: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    season_type: Option<SeasonType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    week: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offense: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    defense: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    conference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offense_conference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    defense_conference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<DivisionClassification>,
}

#[derive(Debug)]
pub struct DriveQueryBuilder<Q> {
    _query: PhantomData<Q>,
    year: Option<i32>,
    season_type: Option<SeasonType>,
    week: Option<i32>,
    team: Option<String>,
    offense: Option<String>,
    defense: Option<String>,
    conference: Option<String>,
    offense_conference: Option<String>,
    defense_conference: Option<String>,
    classification: Option<DivisionClassification>,
}

impl Default for DriveQueryBuilder<NoQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl DriveQueryBuilder<NoQuery> {
    pub fn new() -> Self {
        DriveQueryBuilder {
            _query: PhantomData,
            year: None,
            season_type: None,
            week: None,
            team: None,
            offense: None,
            defense: None,
            conference: None,
            offense_conference: None,
            defense_conference: None,
            classification: None,
        }
    }

    pub fn year(self, year: i32) -> DriveQueryBuilder<ByYear> {
        DriveQueryBuilder {
            _query: PhantomData,
            year: Some(year),
            season_type: self.season_type,
            week: self.week,
            team: self.team,
            offense: self.offense,
            defense: self.defense,
            conference: self.conference,
            offense_conference: self.offense_conference,
            defense_conference: self.defense_conference,
            classification: self.classification,
        }
    }
}

impl DriveQueryBuilder<ByYear> {
    pub fn build(self) -> DriveQuery {
        DriveQuery {
            year: self.year.unwrap(),
            season_type: self.season_type,
            week: self.week,
            team: self.team,
            offense: self.offense,
            defense: self.defense,
            conference: self.conference,
            offense_conference: self.offense_conference,
            defense_conference: self.defense_conference,
            classification: self.classification,
        }
    }
}

impl<Q> DriveQueryBuilder<Q> {
    pub fn season_type(mut self, season_type: impl IntoOptional<SeasonType>) -> Self {
        self.season_type = season_type.into_optional();
        self
    }
    pub fn week(mut self, week: impl IntoOptional<i32>) -> Self {
        self.week = week.into_optional();
        self
    }
    pub fn team(mut self, team: impl IntoOptionalString) -> Self {
        self.team = team.into_optional_string();
        self
    }
    pub fn offense(mut self, offense: impl IntoOptionalString) -> Self {
        self.offense = offense.into_optional_string();
        self
    }
    pub fn defense(mut self, defense: impl IntoOptionalString) -> Self {
        self.defense = defense.into_optional_string();
        self
    }
    pub fn conference(mut self, conference: impl IntoOptionalString) -> Self {
        self.conference = conference.into_optional_string();
        self
    }
    pub fn offense_conference(mut self, offense_conference: impl IntoOptionalString) -> Self {
        self.offense_conference = offense_conference.into_optional_string();
        self
    }
    pub fn defense_conference(mut self, defense_conference: impl IntoOptionalString) -> Self {
        self.defense_conference = defense_conference.into_optional_string();
        self
    }
    pub fn classification(
        mut self,
        classification: impl IntoOptional<DivisionClassification>,
    ) -> Self {
        self.classification = classification.into_optional();
        self
    }
}
