use crate::models::cfb::season_type::SeasonType;
use serde::Serialize;

pub struct NoQuery;
pub struct ByYear(i32);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingQuery {
    pub year: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season_type: Option<SeasonType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week: Option<i32>,
}

impl<Y> RankingQueryBuilder<Y> {
    pub fn season_type(mut self, season_type: SeasonType) -> Self {
        self.season_type = Some(season_type);
        self
    }

    pub fn week(mut self, week: i32) -> Self {
        self.week = Some(week);
        self
    }
}

pub struct RankingQueryBuilder<Y> {
    pub year: Y,
    pub season_type: Option<SeasonType>,
    pub week: Option<i32>,
}

impl RankingQueryBuilder<NoQuery> {
    pub fn new() -> Self {
        RankingQueryBuilder {
            year: NoQuery,
            season_type: None,
            week: None,
        }
    }

    pub fn year(self, year: i32) -> RankingQueryBuilder<ByYear> {
        RankingQueryBuilder {
            year: ByYear(year),
            season_type: self.season_type,
            week: self.week,
        }
    }
}

impl Default for RankingQueryBuilder<NoQuery> {
    fn default() -> Self {
        Self::new()
    }
}

impl RankingQueryBuilder<ByYear> {
    pub fn build(self) -> RankingQuery {
        RankingQuery {
            year: self.year.0,
            season_type: self.season_type,
            week: self.week,
        }
    }
}
