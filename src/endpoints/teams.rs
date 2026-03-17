use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::entity::roster_player::RosterPlayer;
use crate::models::cfb::entity::team::Team;
use crate::models::cfb::query::roster_query::RosterQuery;
use crate::models::cfb::query::team_params::TeamQuery;

impl CfbdClient {
    pub async fn get_teams(&self, params: &TeamQuery) -> Result<Vec<Team>, CFBDError> {
        self.get("teams", Some(params)).await
    }

    pub async fn get_roster(&self, params: &RosterQuery) -> Result<Vec<RosterPlayer>, CFBDError> {
        self.get("roster", Some(params)).await
    }
}
