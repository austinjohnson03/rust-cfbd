use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::poll_week::PollWeek;
use crate::models::cfb::ranking_query::RankingQuery;

impl CfbdClient {
    pub async fn get_rankings(&self, params: &RankingQuery) -> Result<Vec<PollWeek>, CFBDError> {
        self.get("rankings", Some(params)).await
    }
}
