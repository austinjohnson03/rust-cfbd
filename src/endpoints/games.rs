use crate::client::CfbdClient;
use crate::error::CFBDError;
use crate::models::cfb::game::Game;
use crate::models::cfb::get_game_params::{ById, ByYear, GetGameParams};
use serde::Serialize;

pub trait GameQuery: Serialize {}
impl GameQuery for GetGameParams<ById> {}
impl GameQuery for GetGameParams<ByYear> {}

impl CfbdClient {
    pub async fn get_games<P: GameQuery>(&self, params: &P) -> Result<Vec<Game>, CFBDError> {
        self.get("games", Some(params)).await
    }
}
