use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::entity::game::Game;
use crate::models::cfb::entity::game_media::GameMedia;
use crate::models::cfb::query::game_media_query::GameMediaQuery;
use crate::models::cfb::query::game_query::GameQuery;

impl CfbdClient {
    pub async fn get_games(&self, params: &GameQuery) -> Result<Vec<Game>, CFBDError> {
        self.get("games", Some(params)).await
    }

    pub async fn get_game_media(
        &self,
        params: &GameMediaQuery,
    ) -> Result<Vec<GameMedia>, CFBDError> {
        self.get("games/media", Some(params)).await
    }
}
