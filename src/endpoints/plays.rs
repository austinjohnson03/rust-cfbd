use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::entity::play::Play;
use crate::models::cfb::query::play_query::PlayQuery;

impl CfbdClient {
    pub async fn get_plays(&self, params: &PlayQuery) -> Result<Vec<Play>, CFBDError> {
        self.get("plays", Some(params)).await
    }
}
