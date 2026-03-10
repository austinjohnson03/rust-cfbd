use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::team::Team;
use crate::models::cfb::team_params::TeamQuery;

impl CfbdClient {
    pub async fn get_teams(&self, params: &TeamQuery) -> Result<Vec<Team>, CFBDError> {
        self.get("teams", Some(params)).await
    }
}
