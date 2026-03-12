use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::stat_season_query::StatSeasonQuery;
use crate::models::cfb::team_stat::TeamStat;

impl CfbdClient {
    pub async fn get_season_stats(
        &self,
        params: &StatSeasonQuery,
    ) -> Result<Vec<TeamStat>, CFBDError> {
        self.get("stats/season", Some(params)).await
    }
}
