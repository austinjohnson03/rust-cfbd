use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::entity::player_stats::PlayerStat;
use crate::models::cfb::entity::team_stat::TeamStat;
use crate::models::cfb::query::stat_season_query::StatSeasonQuery;
use crate::models::cfb::query::stats_player_season_query::StatsPlayerSeasonQuery;

impl CfbdClient {
    pub async fn get_season_stats(
        &self,
        params: &StatSeasonQuery,
    ) -> Result<Vec<TeamStat>, CFBDError> {
        self.get("stats/season", Some(params)).await
    }

    pub async fn get_player_season_stats(
        &self,
        params: &StatsPlayerSeasonQuery,
    ) -> Result<Vec<PlayerStat>, CFBDError> {
        self.get("stats/player/season", Some(params)).await
    }
}
