use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::info::UserInfo;

impl CfbdClient {
    pub async fn get_user_info(&self) -> Result<UserInfo, CFBDError> {
        self.get::<UserInfo, ()>("info", &()).await
    }
}
