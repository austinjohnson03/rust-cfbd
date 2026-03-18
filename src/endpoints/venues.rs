use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::entity::venue::Venue;

impl CfbdClient {
    pub async fn get_venues(&self) -> Result<Vec<Venue>, CFBDError> {
        self.get::<Vec<Venue>, ()>("venues", None).await
    }
}
