use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::drive::Drive;
use crate::models::cfb::drive_query::DriveQuery;

impl CfbdClient {
    pub async fn get_drives(&self, params: &DriveQuery) -> Result<Vec<Drive>, CFBDError> {
        self.get("drives", Some(params)).await
    }
}
