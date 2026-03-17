use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::entity::game::Game;
use crate::models::cfb::entity::game_media::GameMedia;
use crate::models::cfb::query::game_media_params::GameMediaQuery;
use crate::models::cfb::query::get_game_params::{ById, ByYear, GetGameParams};
use serde::Serialize;

pub trait GameQuery: Serialize {}
impl GameQuery for GetGameParams<ById> {}
impl GameQuery for GetGameParams<ByYear> {}

impl CfbdClient {
    pub async fn get_games<P: GameQuery>(&self, params: &P) -> Result<Vec<Game>, CFBDError> {
        self.get("games", Some(params)).await
    }

    pub async fn get_game_media(
        &self,
        params: &GameMediaQuery,
    ) -> Result<Vec<GameMedia>, CFBDError> {
        self.get("games/media", Some(params)).await
    }
}
