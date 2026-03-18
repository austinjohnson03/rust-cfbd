use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::entity::conference::Conference;

impl CfbdClient {
    pub async fn get_conferences(&self) -> Result<Vec<Conference>, CFBDError> {
        self.get::<Vec<Conference>, ()>("conferences", None).await
    }
}
