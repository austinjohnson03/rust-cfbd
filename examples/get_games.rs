use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use rust_cfbd::models::cfb::entity::division_classification::DivisionClassification;
use rust_cfbd::models::cfb::entity::game::Game;
use rust_cfbd::models::cfb::query::game_query::GameQueryBuilder;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let params_id_example = GameQueryBuilder::new().id(401760371).build();
    let params_year_example = GameQueryBuilder::new()
        .year(2024)
        .week(1)
        .classification(DivisionClassification::Fbs)
        .build();
    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp_id_example: Vec<Game> = client.get_games(&params_id_example).await.unwrap();
    let json_id_example = serde_json::to_string_pretty(&resp_id_example).unwrap();

    let resp_year_example: Vec<Game> = client.get_games(&params_year_example).await.unwrap();
    let json_year_example = serde_json::to_string_pretty(&resp_year_example).unwrap();

    println!("{}", json_id_example);
    println!("{}", json_year_example);
}
