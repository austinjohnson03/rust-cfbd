use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::recruit::Recruit;
use crate::models::cfb::recruit_player_query::RecruitPlayerQuery;
use crate::models::cfb::recruit_team_query::RecruitTeamQuery;
use crate::models::cfb::team_recruiting_ranking::TeamRecruitingRanking;

impl CfbdClient {
    pub async fn get_recruiting_players(
        &self,
        params: &RecruitPlayerQuery,
    ) -> Result<Vec<Recruit>, CFBDError> {
        self.get("recruiting/players", Some(params)).await
    }

    pub async fn get_recruiting_teams(
        &self,
        params: &RecruitTeamQuery,
    ) -> Result<Vec<TeamRecruitingRanking>, CFBDError> {
        self.get("recruiting/teams", Some(params)).await
    }
}
