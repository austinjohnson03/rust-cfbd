use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::entity::coach::Coach;
use crate::models::cfb::query::coach_query::CoachQuery;

impl CfbdClient {
    pub async fn get_coaches(&self, params: &CoachQuery) -> Result<Vec<Coach>, CFBDError> {
        self.get("coaches", Some(params)).await
    }
}
