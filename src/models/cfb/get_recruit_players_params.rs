use serde::Serialize;
use std::marker::PhantomData;

use crate::models::cfb::recruit_type::RecruitType;

pub struct NoQuery;

pub struct ByYear;

pub struct ByTeam;

#[derive(Debug, Serialize)]
pub struct GetRecruitPlayersParams<Q> {
    #[serde(skip)]
    _query: PhantomData<Q>,

    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<RecruitType>,
}

impl Default for GetRecruitPlayersParams<NoQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl GetRecruitPlayersParams<NoQuery> {
    pub fn new() -> Self {
        Self {
            _query: PhantomData,
            year: None,
            team: None,
            position: None,
            state: None,
            classification: None,
        }
    }

    pub fn year(self, year: u32) -> GetRecruitPlayersParams<ByYear> {
        GetRecruitPlayersParams {
            _query: PhantomData,
            year: Some(year),
            team: self.team,
            position: self.position,
            state: self.state,
            classification: self.classification,
        }
    }

    pub fn team(self, team: String) -> GetRecruitPlayersParams<ByTeam> {
        GetRecruitPlayersParams {
            _query: PhantomData,
            year: self.year,
            team: Some(team),
            position: self.position,
            state: self.state,
            classification: self.classification,
        }
    }
}

impl<Q> GetRecruitPlayersParams<Q> {
    pub fn position(mut self, position: String) -> Self {
        self.position = Some(position);
        self
    }

    pub fn state(mut self, state: String) -> Self {
        self.state = Some(state);
        self
    }

    pub fn classification(mut self, classification: RecruitType) -> Self {
        self.classification = Some(classification);
        self
    }
}
