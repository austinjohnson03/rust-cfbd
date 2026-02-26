use dotenvy::dotenv;
use rust_cfbd::client::CfbdClient;
use rust_cfbd::config::Config;
use rust_cfbd::models::cfb::division_classification::DivisionClassification;
use rust_cfbd::models::cfb::game::Game;
use rust_cfbd::models::cfb::get_game_params::GetGameParams;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let params_id_example = GetGameParams::new().id(401760371);
    let params_year_example = GetGameParams::new()
        .year(2024)
        .week(1)
        .classification(DivisionClassification::Fbs);
    let api_key = env::var("CFBD_API_KEY").expect("CFBD_API_KEY not set");

    let client = CfbdClient::new(Config::new(api_key));

    let resp_id_example: Vec<Game> = client.get("games", &params_id_example).await.unwrap();
    let json_id_example = serde_json::to_string_pretty(&resp_id_example).unwrap();

    let resp_year_example: Vec<Game> = client.get("games", &params_year_example).await.unwrap();
    let json_year_example = serde_json::to_string_pretty(&resp_year_example).unwrap();

    println!("{}", json_id_example);
    println!("{}", json_year_example);
}
