use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::recruit::Recruit;
use crate::models::cfb::recruit_player_query::RecruitPlayerQuery;

impl CfbdClient {
    pub async fn get_recruiting_players(
        &self,
        params: &RecruitPlayerQuery,
    ) -> Result<Vec<Recruit>, CFBDError> {
        self.get("recruiting/players", Some(params)).await
    }
}
