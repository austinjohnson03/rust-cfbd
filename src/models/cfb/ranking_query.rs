use serde::Serialize;
use crate::models::cfb::season_type::SeasonType;

pub struct NoQuery;
pub struct ByYear(i32);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingQuery {
    pub year: i32,
    pub season_type: Option<SeasonType>,
    pub week: Option<i32>,
}

pub struct RankingQueryBuilder {
    pub year: Option<i32>,
    pub season_type: Option<SeasonType>,
    pub week: Option<i32>,
}
