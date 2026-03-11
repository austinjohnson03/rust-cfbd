use serde::Serialize;

use crate::models::cfb::{
    division_classification::DivisionClassification, season_type::SeasonType,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayQuery {
    pub year: i32,
    pub week: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offense: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offense_conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense_conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season_type: Option<SeasonType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<DivisionClassification>,
}

pub struct NoYear;
pub struct ByYear(i32);

pub struct NoWeek;
pub struct ByWeek(i32);

pub struct PlayQueryBuilder<Y, W> {
    year: Y,
    week: W,
    team: Option<String>,
    offense: Option<String>,
    defense: Option<String>,
    offense_conference: Option<String>,
    defense_conference: Option<String>,
    conference: Option<String>,
    play_type: Option<String>,
    season_type: Option<SeasonType>,
    classification: Option<DivisionClassification>,
}

impl PlayQueryBuilder<NoYear, NoWeek> {
    pub fn new() -> Self {
        PlayQueryBuilder {
            year: NoYear,
            week: NoWeek,
            team: None,
            offense: None,
            defense: None,
            offense_conference: None,
            defense_conference: None,
            conference: None,
            play_type: None,
            season_type: None,
            classification: None,
        }
    }
}

impl Default for PlayQueryBuilder<NoYear, NoWeek> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Y, W> PlayQueryBuilder<Y, W> {
    pub fn team(mut self, team: impl Into<String>) -> Self {
        self.team = Some(team.into());
        self
    }

    pub fn offense(mut self, offense: impl Into<String>) -> Self {
        self.offense = Some(offense.into());
        self
    }

    pub fn defense(mut self, defense: impl Into<String>) -> Self {
        self.defense = Some(defense.into());
        self
    }

    pub fn offense_conference(mut self, offense_conference: impl Into<String>) -> Self {
        self.offense_conference = Some(offense_conference.into());
        self
    }

    pub fn defense_conference(mut self, defense_conference: impl Into<String>) -> Self {
        self.defense_conference = Some(defense_conference.into());
        self
    }

    pub fn conference(mut self, conference: impl Into<String>) -> Self {
        self.conference = Some(conference.into());
        self
    }

    pub fn play_type(mut self, play_type: impl Into<String>) -> Self {
        self.play_type = Some(play_type.into());
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
}

impl<W> PlayQueryBuilder<NoYear, W> {
    pub fn year(self, year: i32) -> PlayQueryBuilder<ByYear, W> {
        PlayQueryBuilder {
            year: ByYear(year),
            week: self.week,
            team: self.team,
            offense: self.offense,
            defense: self.defense,
            offense_conference: self.offense_conference,
            defense_conference: self.defense_conference,
            conference: self.conference,
            play_type: self.play_type,
            season_type: self.season_type,
            classification: self.classification,
        }
    }
}

impl<Y> PlayQueryBuilder<Y, NoWeek> {
    pub fn week(self, week: i32) -> PlayQueryBuilder<Y, ByWeek> {
        PlayQueryBuilder {
            year: self.year,
            week: ByWeek(week),
            team: self.team,
            offense: self.offense,
            defense: self.defense,
            offense_conference: self.offense_conference,
            defense_conference: self.defense_conference,
            conference: self.conference,
            play_type: self.play_type,
            season_type: self.season_type,
            classification: self.classification,
        }
    }
}

impl PlayQueryBuilder<ByYear, ByWeek> {
    pub fn build(self) -> PlayQuery {
        PlayQuery {
            year: self.year.0,
            week: self.week.0,
            team: self.team,
            offense: self.offense,
            defense: self.defense,
            offense_conference: self.offense_conference,
            defense_conference: self.defense_conference,
            conference: self.conference,
            play_type: self.play_type,
            season_type: self.season_type,
            classification: self.classification,
        }
    }
}
